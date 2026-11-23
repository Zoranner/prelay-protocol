use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderProtocol {
    ChatCompletions,
    Responses,
    ImagesGenerations,
}

impl ProviderProtocol {
    pub const ORDERED: [Self; 3] = [
        Self::ChatCompletions,
        Self::Responses,
        Self::ImagesGenerations,
    ];
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderAuthScheme {
    Bearer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderProtocolBaseUrl {
    pub protocol: ProviderProtocol,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogLanguageModelResponse {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub reasoning_efforts: Option<Vec<String>>,
    pub default_reasoning_effort: Option<String>,
    pub context_window: Option<u64>,
    pub max_context_window: Option<u64>,
    pub effective_context_window_percent: Option<u8>,
    pub input_modalities: Option<Vec<String>>,
    pub supports_parallel_tool_calls: Option<bool>,
    pub supports_reasoning_summaries: Option<bool>,
    pub supports_image_detail_original: Option<bool>,
    pub support_verbosity: Option<bool>,
    pub default_verbosity: Option<String>,
    pub apply_patch_tool_type: Option<String>,
    pub web_search_tool_type: Option<String>,
    /// Codex `tool_search` 能力；与 `web_search_tool_type` 无关，缺省表示不支持。
    pub supports_search_tool: Option<bool>,
    pub truncation_policy: Option<CatalogTruncationPolicyResponse>,
    pub reasoning_summary_format: Option<String>,
    pub default_reasoning_summary: Option<String>,
    pub shell_type: Option<String>,
    pub visibility: Option<String>,
    pub supported_in_api: Option<bool>,
    pub priority: Option<u32>,
    pub base_instructions: Option<String>,
    pub experimental_supported_tools: Option<Vec<String>>,
    pub minimal_client_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogTruncationPolicyResponse {
    pub mode: String,
    pub limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogImageGenerationModelResponse {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub input_modalities: Option<Vec<String>>,
    pub output_modalities: Option<Vec<String>>,
    pub sizes: Option<Vec<String>>,
    pub quality_options: Option<Vec<String>>,
    pub background_options: Option<Vec<String>>,
    pub output_formats: Option<Vec<String>>,
    pub supports_editing: Option<bool>,
    pub supports_mask: Option<bool>,
    pub supports_reference_images: Option<bool>,
    pub visibility: Option<String>,
    pub supported_in_api: Option<bool>,
    pub priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogProviderResponse {
    pub id: String,
    pub name: String,
    pub auth_scheme: ProviderAuthScheme,
    pub base_url: String,
    pub protocols: Vec<ProviderProtocol>,
    pub protocol_base_urls: Vec<ProviderProtocolBaseUrl>,
    pub language_models: Vec<String>,
    pub image_generation_models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCatalogResponse {
    pub language_models: Vec<CatalogLanguageModelResponse>,
    pub image_generation_models: Vec<CatalogImageGenerationModelResponse>,
    pub providers: Vec<CatalogProviderResponse>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderVisibility {
    Private,
    Selected,
    All,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCapabilityOverrides {
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
    pub images_generations: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateProviderRequest {
    pub name: String,
    pub provider_type: String,
    pub base_url: String,
    pub api_key: String,
    pub capabilities: Option<ProviderCapabilityOverrides>,
    /// 该供应商启用的目录模型 id；缺省表示启用该目录条目的全部模型。
    pub models: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateProviderRequest {
    pub name: Option<String>,
    pub provider_type: Option<String>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub capabilities: Option<ProviderCapabilityOverrides>,
    /// 已启用模型清单；缺省表示保持原值。
    pub models: Option<Vec<String>>,
}

/// Provider 唯一读模型：列表、编辑前加载和保存回写共用同一份结构。
///
/// 编辑权限由 `owner_identity_id` 与当前身份比对得出，不另设权限字段；
/// `credential` 只在创建人读取时返回，列表恒为 `None`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderResponse {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub base_url: String,
    pub capabilities: ProviderCapabilityOverrides,
    pub upstream_protocols: Vec<String>,
    /// 该供应商启用的模型清单，与目录条目比对可以判断模型是否已下架。
    pub models: Vec<String>,
    pub created_at: String,
    pub owner_identity_id: String,
    pub owner_display_name: String,
    pub visibility: ProviderVisibility,
    pub credential: Option<ProviderCredential>,
}

/// 创建人可见的上游凭据；`api_key` 供编辑时回填和复制。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCredential {
    pub api_key: String,
    pub api_key_masked: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ShareQuotaWindowKind {
    Day,
    Week,
    Month,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderQuotaMode {
    Own,
    SharedUnlimited,
    SharedLimited,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShareQuotaWindow {
    pub window: ShareQuotaWindowKind,
    pub limit: i64,
    pub accumulate: bool,
    pub reset_time: String,
    pub reset_weekday: Option<u8>,
    pub reset_day: Option<u8>,
}

/// 共享给指定用户时的一条授权。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderSharingUserInput {
    pub identity_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderSharingUserResponse {
    pub identity_id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderSharingResponse {
    pub visibility: ProviderVisibility,
    /// 仅在 `visibility` 为 `selected` 时非空。
    pub users: Vec<ProviderSharingUserResponse>,
    /// 共享额度规则，对所有被共享用户生效；为空表示不限额。
    pub quota: Option<Vec<ShareQuotaWindow>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateProviderSharingRequest {
    pub visibility: ProviderVisibility,
    pub users: Vec<ProviderSharingUserInput>,
    pub quota: Option<Vec<ShareQuotaWindow>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum QuotaWindowKind {
    Custom,
    Rolling,
    Day,
    Week,
    Month,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuotaUnit {
    Tokens,
    Usd,
    Credits,
    Requests,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuotaWindow {
    pub kind: QuotaWindowKind,
    /// 窗口长度；月窗口可以不填
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_secs: Option<i64>,
    /// 统一为已用百分比，0 到 100
    pub used_percent: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resets_at: Option<String>,
    /// 仅共享限额窗口使用，表示剩余 token；个人套餐窗口不填
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining: Option<f64>,
    /// 仅共享限额窗口使用，表示 token 上限
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// 仅共享限额窗口使用，当前为 tokens
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<QuotaUnit>,
    /// 厂商原样的计数，例如 "1.2k / 3k"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuotaBalancePart {
    pub currency: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuotaBalance {
    pub parts: Vec<QuotaBalancePart>,
    pub available: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuotaErrorCode {
    Unsupported,
    Unconfigured,
    Unauthorized,
    Unreachable,
    InvalidResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuotaError {
    pub code: QuotaErrorCode,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProviderQuota {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub windows: Vec<QuotaWindow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<QuotaBalance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<QuotaError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_at: Option<String>,
    #[serde(default)]
    pub stale: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProviderQuotaResponse {
    pub provider_id: String,
    pub owner_identity_id: String,
    pub owner_display_name: String,
    pub mode: ProviderQuotaMode,
    pub quota: ProviderQuota,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderUsageResponse {
    pub total_requests: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_tokens: i64,
    pub latest_used_at: Option<String>,
    pub users: Vec<ProviderUsageUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderUsageUser {
    pub identity_id: String,
    pub display_name: String,
    pub request_count: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_tokens: i64,
    pub latest_used_at: Option<String>,
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
}
