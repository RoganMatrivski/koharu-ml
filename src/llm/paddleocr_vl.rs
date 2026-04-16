use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use image::DynamicImage;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;

use crate::llm::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::llm::Language;

pub struct PaddleOcrVl {
    provider: OpenAiCompatibleProvider,
}

impl PaddleOcrVl {
    pub async fn load(
        _runtime_dir: &Path,
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
            provider,
        })
    }

    pub async fn generate(
        &mut self,
        image: &DynamicImage,
        _max_tokens: usize,
    ) -> Result<String> {
        let mut buffer = std::io::Cursor::new(Vec::new());
        image.write_to(&mut buffer, image::ImageFormat::Png)?;
        let image_base64 = base64::engine::general_purpose::STANDARD.encode(buffer.get_ref());

        self.provider.multimodal(
            "OCR: ",
            Language::English,
            "paddleocr-vl",
            None,
            Some(image_base64),
        ).await
    }
}

pub async fn prefetch() -> Result<()> {
    Ok(())
}
