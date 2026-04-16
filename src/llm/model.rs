use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;

use crate::llm::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::llm::{Language, ModelId};

pub struct Llm {
    model_id: ModelId,
    provider: OpenAiCompatibleProvider,
}

#[derive(Debug, Clone)]
pub struct GenerateOptions {
    pub max_tokens: usize,
    pub temperature: f64,
    pub top_k: Option<usize>,
    pub top_p: Option<f64>,
    pub min_p: Option<f64>,
    pub seed: u64,
    pub split_prompt: bool,
    pub repeat_penalty: f32,
    pub repeat_last_n: usize,
    pub presence_penalty: f32,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            max_tokens: 1000,
            temperature: 0.1,
            top_k: None,
            top_p: None,
            min_p: None,
            seed: 299792458,
            split_prompt: false,
            repeat_penalty: 1.1,
            repeat_last_n: 64,
            presence_penalty: 0.0,
        }
    }
}

impl Llm {
    pub async fn load(
        _runtime_dir: &Path,
        id: ModelId,
        _cpu: bool,
    ) -> Result<Self> {
        let base_url = std::env::var("KOHARU_LOCAL_LLM_URL")
            .unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let http_client = Arc::new(
            ClientBuilder::new(reqwest::Client::new())
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build(),
        );

        let provider = OpenAiCompatibleProvider {
            http_client,
            base_url,
            api_key: None,
            temperature: None,
            max_tokens: None,
        };

        Ok(Self {
            model_id: id,
            provider,
        })
    }

    pub fn id(&self) -> ModelId {
        self.model_id
    }

    pub async fn generate(
        &mut self,
        prompt: &str,
        opts: &GenerateOptions,
        target_language: Language,
        system_prompt: Option<&str>,
    ) -> Result<String> {
        if opts.max_tokens == 0 {
            return Ok(String::new());
        }

        self.provider
            .multimodal(
                prompt,
                target_language,
                &self.model_id.to_string(),
                system_prompt,
                None,
            )
            .await
    }
}

pub fn rate(tokens: usize, duration: Duration) -> f64 {
    if duration.as_secs_f64() > 0.0 {
        tokens as f64 / duration.as_secs_f64()
    } else {
        0.0
    }
}
