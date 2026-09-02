use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderProtocol {
    ChatCompletions,
    Responses,
    AnthropicMessages,
    ImagesGenerations,
}

impl ProviderProtocol {
    pub const ORDERED: [Self; 4] = [
        Self::ChatCompletions,
        Self::Responses,
        Self::AnthropicMessages,
        Self::ImagesGenerations,
    ];
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderAuthScheme {
    Bearer,
    Anthropic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    Text,
    Image,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderProtocolBaseUrl {
    pub protocol: ProviderProtocol,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogModelResponse {
    pub id: String,
    pub display_name: String,
    pub model_type: ModelType,
    pub reasoning_efforts: Vec<String>,
    pub default_reasoning_effort: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogProviderResponse {
    pub id: String,
    pub name: String,
    pub auth_scheme: ProviderAuthScheme,
    pub base_url: String,
    pub protocols: Vec<ProviderProtocol>,
    pub protocol_base_urls: Vec<ProviderProtocolBaseUrl>,
    pub models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCatalogResponse {
    pub models: Vec<CatalogModelResponse>,
    pub providers: Vec<CatalogProviderResponse>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCapabilityOverrides {
    pub upstream_protocols: Option<Vec<String>>,
    pub protocol_base_urls: Option<ProviderProtocolBaseUrls>,
    pub tool_calls: Option<bool>,
    pub reasoning: Option<bool>,
    pub tool_choice: Option<bool>,
    pub parallel_tool_calls: Option<bool>,
    pub system_messages: Option<bool>,
    pub structured_outputs: Option<bool>,
    pub streaming_usage: Option<bool>,
    pub max_context_tokens: Option<i64>,
    pub max_output_tokens: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderProtocolBaseUrls {
    pub responses: Option<String>,
    pub openai: Option<String>,
    pub anthropic: Option<String>,
    pub images_generations: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateProviderRequest {
    pub name: String,
    pub provider_type: String,
    pub base_url: String,
    pub api_key: String,
    pub capabilities: Option<ProviderCapabilityOverrides>,
    pub models: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateProviderRequest {
    pub name: Option<String>,
    pub provider_type: Option<String>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub capabilities: Option<ProviderCapabilityOverrides>,
    pub models: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderModelResponse {
    pub id: String,
    pub provider_id: String,
    pub model_name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderResponse {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub base_url: String,
    pub api_key: String,
    pub api_key_masked: String,
    pub capabilities: ProviderCapabilityOverrides,
    pub upstream_protocols: Vec<String>,
    pub models: Vec<ProviderModelResponse>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestProviderProtocolRequest {
    pub protocol: String,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderOperationRequest {
    pub provider_type: String,
    pub base_url: String,
    pub api_key: String,
    pub protocol: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderOperationResponse {
    pub ok: bool,
    pub protocol: Option<String>,
    pub latency_ms: Option<i64>,
    pub first_token_ms: Option<i64>,
    pub error: Option<String>,
    pub models: Option<Vec<String>>,
}
