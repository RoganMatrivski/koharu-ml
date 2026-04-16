use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use candle_transformers::models::paddleocr_vl as hf_model;
use image::DynamicImage;


use crate::llm::safe::context::params::LlamaContextParams;
use crate::llm::safe::llama_backend::LlamaBackend;
use crate::llm::safe::llama_batch::LlamaBatch;
use crate::llm::safe::model::params::LlamaModelParams;
use crate::llm::safe::model::{AddBos, LlamaModel};
use crate::llm::safe::mtmd::{MtmdContext, MtmdParams};
use crate::llm::safe::sampling::LlamaSampler;
use crate::llm::safe::token::LlamaToken;

const HF_REPO: &str = "PaddlePaddle/PaddleOCR-VL-1.5-GGUF";
const DEFAULT_GPU_LAYERS: u32 = 1000;
const MAX_UBATCH: u32 = 512;

struct ModelFiles {
    model: PathBuf,
    mmproj: PathBuf,
}

pub struct PaddleOcrVl {
    backend: Arc<LlamaBackend>,
    model: LlamaModel,
    chat_template: String,
    bos_token: String,
    eos_token_text: String,
    mtmd: MtmdContext,
    eos_token: LlamaToken,
}

impl PaddleOcrVl {
    pub async fn load(
        runtime_dir: &Path,
        cpu: bool,
        backend: Arc<LlamaBackend>,
    ) -> Result<Self> {
        let files = download_model_files().await?;
        tokio::task::spawn_blocking(move || Self::load_from_files(runtime_dir, files, cpu, backend))
            .await
            .context("failed to join PaddleOCR-VL loading task")?
    }

    pub fn load_from_dir(
        runtime_dir: &Path,
        dir: impl AsRef<Path>,
        cpu: bool,
        backend: Arc<LlamaBackend>,
    ) -> Result<Self> {
        let files = resolve_local_model_files(dir.as_ref())?;
        Self::load_from_files(runtime_dir, files, cpu, backend)
    }

    fn load_from_files(
        runtime_dir: &Path,
        files: ModelFiles,
        cpu: bool,
        backend: Arc<LlamaBackend>,
    ) -> Result<Self> {
        crate::llm::sys::initialize(runtime_dir)
            .context("failed to initialize llama.cpp runtime bindings")?;

        let model_params = model_params(cpu, backend.as_ref());
        let model = LlamaModel::load_from_file(backend.as_ref(), &files.model, &model_params)
            .with_context(|| format!("unable to load model from `{}`", files.model.display()))?;
        let eos_token = model.token_eos();

        let chat_template = model
            .meta_val_str("tokenizer.ggml.chat_template")
            .or_else(|_| model.meta_val_str("tokenizer.chat_template"))
            .context("missing chat template in GGUF metadata")?;

        let bos_token = token_text(&model, model.token_bos());
        let eos_token_text = token_text(&model, eos_token);

        let mtmd = MtmdContext::new(
            backend.as_ref(),
            &files.mmproj,
            &MtmdParams::default().with_n_gpu_layers(if cpu { 0 } else { DEFAULT_GPU_LAYERS }),
        )
        .context("failed to create multimodal context (MTMD)")?;

        Ok(Self {
            backend,
            model,
            chat_template,
            bos_token,
            eos_token_text,
            mtmd,
            eos_token,
        })
    }

    pub fn generate(
        &mut self,
        image: &DynamicImage,
        max_tokens: usize,
    ) -> Result<String> {
        if max_tokens == 0 {
            return Ok(String::new());
        }

        let prompt = "OCR: ";
        let messages = vec![crate::llm::prompt::ChatMessage {
            role: crate::llm::prompt::ChatRole::User,
            content: format!("<|vision_start|><|image_pad|><|vision_end|>{}", prompt),
        }];
        let mut renderer = crate::llm::prompt::PromptRenderer::new(
            crate::llm::ModelId::VntlLlama3_8Bv2, // dummy
            &self.chat_template,
            self.bos_token.clone(),
            self.eos_token_text.clone(),
        );
        let formatted_prompt = renderer.format_messages(messages, crate::llm::Language::English, None)?;

        let prompt_tokens = self
            .model
            .str_to_token(&formatted_prompt, AddBos::Never)
            .context("failed to tokenize prompt")?;

        let mut ctx = self
            .model
            .new_context(
                self.backend.as_ref(),
                context_params(prompt_tokens.len(), max_tokens)?,
            )
            .context("unable to create llama.cpp context")?;

        let visual_config = hf_model::Config::default().vision_config;
        let image_embd = self.mtmd.encode_image(
            &mut ctx,
            image,
            &visual_config,
        )?;

        let mut batch = LlamaBatch::new(prompt_tokens.len() + image_embd.n_tokens() as usize, 1);
        let mut pos = 0i32;

        let image_token_id = self.model.str_to_token("<|image_pad|>", AddBos::Never)?[0];

        for token in prompt_tokens {
            if token == image_token_id {
                ctx.mtmd_eval(&mut batch, &image_embd, pos, &[0])?;
                pos += image_embd.n_tokens() as i32;
            } else {
                batch.add(token, pos, &[0], false)?;
                pos += 1;
            }
        }

        ctx.decode(&mut batch)?;

        let mut sampler = LlamaSampler::greedy();
        let mut next_token = sampler.sample(&ctx, batch.n_tokens() - 1);
        let mut generated = String::new();
        let mut decoder = encoding_rs::UTF_8.new_decoder();

        for _ in 0..max_tokens {
            if next_token == self.eos_token || self.model.is_eog_token(next_token) {
                break;
            }

            let piece = self.model.token_to_piece(next_token, &mut decoder, true, None)?;
            generated.push_str(&piece);

            batch.clear();
            batch.add(next_token, pos, &[0], true)?;
            ctx.decode(&mut batch)?;
            pos += 1;
            next_token = sampler.sample(&ctx, 0);
        }

        Ok(generated)
    }
}

fn model_params(cpu: bool, backend: &LlamaBackend) -> LlamaModelParams {
    if !cpu && backend.supports_gpu_offload() {
        LlamaModelParams::default().with_n_gpu_layers(DEFAULT_GPU_LAYERS)
    } else {
        LlamaModelParams::default()
    }
}

fn context_params(prompt_tokens: usize, max_tokens: usize) -> Result<LlamaContextParams> {
    let required_ctx = prompt_tokens
        .saturating_add(max_tokens)
        .saturating_add(1)
        .max(1);
    let n_ctx = NonZeroU32::new(u32::try_from(required_ctx).context("context size exceeds u32")?)
        .expect("required context is always non-zero");
    let n_batch = u32::try_from(prompt_tokens.max(1)).context("prompt batch size exceeds u32")?;
    let n_ubatch = n_batch.min(MAX_UBATCH);

    Ok(LlamaContextParams::default()
        .with_n_ctx(Some(n_ctx))
        .with_n_batch(n_batch)
        .with_n_ubatch(n_ubatch))
}

fn token_text(model: &LlamaModel, token: LlamaToken) -> String {
    let mut decoder = encoding_rs::UTF_8.new_decoder();
    match model.token_to_piece(token, &mut decoder, true, None) {
        Ok(piece) if !piece.is_empty() => piece,
        _ => token.to_string(),
    }
}

fn resolve_local_model_files(dir: &Path) -> Result<ModelFiles> {
    Ok(ModelFiles {
        model: dir.join("paddleocr-vl-1.5-q4_k_m.gguf"),
        mmproj: dir.join("mmproj-paddleocr-vl-1.5-f16.gguf"),
    })
}

pub async fn prefetch() -> Result<()> {
    download_model_files().await?;
    Ok(())
}

async fn download_model_files() -> Result<ModelFiles> {
    let model = crate::ml::loading::hf_download(HF_REPO, "paddleocr-vl-1.5-q4_k_m.gguf").await?;
    let mmproj = crate::ml::loading::hf_download(HF_REPO, "mmproj-paddleocr-vl-1.5-f16.gguf").await?;
    Ok(ModelFiles { model, mmproj })
}
