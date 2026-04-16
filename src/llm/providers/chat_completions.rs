use std::sync::Arc;

use reqwest_middleware::ClientWithMiddleware;
use serde::Serialize;

use super::ensure_provider_success;

pub enum ChatCompletionsAuth {
    None,
    Bearer(String),
}

pub struct ChatCompletionsRequest {
    pub provider: &'static str,
    pub endpoint: String,
    pub auth: ChatCompletionsAuth,
    pub model: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u32>,
    pub image_base64: Option<String>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ChatMessageContent {
    Text(String),
    Multimodal(Vec<MultimodalContent>),
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MultimodalContent {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize)]
struct ChatMessage {
    role: &'static str,
    content: ChatMessageContent,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

pub async fn send_chat_completion(
    http_client: Arc<ClientWithMiddleware>,
    request: ChatCompletionsRequest,
) -> anyhow::Result<String> {
    let user_content = if let Some(image_b64) = request.image_base64 {
        ChatMessageContent::Multimodal(vec![
            MultimodalContent::ImageUrl {
                image_url: ImageUrl {
                    url: format!("data:image/png;base64,{}", image_b64),
                },
            },
            MultimodalContent::Text {
                text: request.user_prompt,
            },
        ])
    } else {
        ChatMessageContent::Text(request.user_prompt)
    };

    let body = ChatRequest {
        model: &request.model,
        messages: vec![
            ChatMessage {
                role: "system",
                content: ChatMessageContent::Text(request.system_prompt),
            },
            ChatMessage {
                role: "user",
                content: user_content,
            },
        ],
        temperature: request.temperature,
        max_tokens: request.max_tokens,
    };

    let mut http_request = http_client.post(&request.endpoint);
    if let ChatCompletionsAuth::Bearer(api_key) = request.auth {
        http_request = http_request.bearer_auth(api_key);
    }

    let response = http_request
        .header("content-type", "application/json")
        .body(serde_json::to_vec(&body)?)
        .send()
        .await?;

    let resp: serde_json::Value = ensure_provider_success(request.provider, response)
        .await?
        .json()
        .await?;

    resp["choices"][0]["message"]["content"]
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow::anyhow!("{} returned no content", request.provider))
}
