use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolErrorCode {
    IdentityAlreadyRegistered,
    ClientUpdateUnavailable,
    ExtensionCatalogUnavailable,
    ExtensionContentInvalid,
    ExtensionNotFound,
    ExtensionVersionNotFound,
    ExtensionInstallUnsupported,
    InvalidProviderSharing,
    ProviderSharingNotAllowed,
    ProviderNotVisible,
    ProviderNotUsable,
    ProviderRouteUnavailable,
    ProviderUpstreamUnavailable,
    ProviderModelRetired,
    ProviderTypeUnsupported,
    EndpointModelDuplicate,
    EndpointModelNotInProviderModels,
    InvalidCredential,
    NotFound,
    ValidationFailed,
    Internal,
}

impl ProtocolErrorCode {
    /// 管理 API 会返回的全部稳定错误码。
    ///
    /// 新增变体必须同时加入这里；稳定性测试按该列表逐项断言，未登记即失败。
    pub const ALL: [Self; 21] = [
        Self::IdentityAlreadyRegistered,
        Self::ClientUpdateUnavailable,
        Self::ExtensionCatalogUnavailable,
        Self::ExtensionContentInvalid,
        Self::ExtensionNotFound,
        Self::ExtensionVersionNotFound,
        Self::ExtensionInstallUnsupported,
        Self::InvalidProviderSharing,
        Self::ProviderSharingNotAllowed,
        Self::ProviderNotVisible,
        Self::ProviderNotUsable,
        Self::ProviderRouteUnavailable,
        Self::ProviderUpstreamUnavailable,
        Self::ProviderModelRetired,
        Self::ProviderTypeUnsupported,
        Self::EndpointModelDuplicate,
        Self::EndpointModelNotInProviderModels,
        Self::InvalidCredential,
        Self::NotFound,
        Self::ValidationFailed,
        Self::Internal,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityAlreadyRegistered => "identity_already_registered",
            Self::ClientUpdateUnavailable => "client_update_unavailable",
            Self::ExtensionCatalogUnavailable => "extension_catalog_unavailable",
            Self::ExtensionContentInvalid => "extension_content_invalid",
            Self::ExtensionNotFound => "extension_not_found",
            Self::ExtensionVersionNotFound => "extension_version_not_found",
            Self::ExtensionInstallUnsupported => "extension_install_unsupported",
            Self::InvalidProviderSharing => "invalid_provider_sharing",
            Self::ProviderSharingNotAllowed => "provider_sharing_not_allowed",
            Self::ProviderNotVisible => "provider_not_visible",
            Self::ProviderNotUsable => "provider_not_usable",
            Self::ProviderRouteUnavailable => "provider_route_unavailable",
            Self::ProviderUpstreamUnavailable => "provider_upstream_unavailable",
            Self::ProviderModelRetired => "provider_model_retired",
            Self::ProviderTypeUnsupported => "provider_type_unsupported",
            Self::EndpointModelDuplicate => "endpoint_model_duplicate",
            Self::EndpointModelNotInProviderModels => "endpoint_model_not_in_provider_models",
            Self::InvalidCredential => "invalid_credential",
            Self::NotFound => "not_found",
            Self::ValidationFailed => "validation_failed",
            Self::Internal => "internal",
        }
    }
}

/// 推理接口（`/v1/*`）的错误码。
///
/// 与 [`ProtocolErrorCode`] 分开维护：`/v1` 面向 OpenAI 兼容客户端，
/// 码值和 `type` 遵循 OpenAI 错误体的语义，不复用管理面的业务码。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum V1ErrorCode {
    InvalidRequestError,
    UnsupportedParameter,
    PreviousResponseNotFound,
    PayloadTooLarge,
    InvalidApiKey,
    NotFound,
    MethodNotAllowed,
    PermissionDenied,
    ServiceUnavailable,
    UpstreamStatus,
    UpstreamConnect,
    UpstreamInvalidResponse,
    SharedQuotaExhausted,
    RecordingUnavailable,
    Internal,
}

impl V1ErrorCode {
    pub const ALL: [Self; 15] = [
        Self::InvalidRequestError,
        Self::UnsupportedParameter,
        Self::PreviousResponseNotFound,
        Self::PayloadTooLarge,
        Self::InvalidApiKey,
        Self::NotFound,
        Self::MethodNotAllowed,
        Self::PermissionDenied,
        Self::ServiceUnavailable,
        Self::UpstreamStatus,
        Self::UpstreamConnect,
        Self::UpstreamInvalidResponse,
        Self::SharedQuotaExhausted,
        Self::RecordingUnavailable,
        Self::Internal,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequestError => "invalid_request_error",
            Self::UnsupportedParameter => "unsupported_parameter",
            Self::PreviousResponseNotFound => "previous_response_not_found",
            Self::PayloadTooLarge => "payload_too_large",
            Self::InvalidApiKey => "invalid_api_key",
            Self::NotFound => "not_found",
            Self::MethodNotAllowed => "method_not_allowed",
            Self::PermissionDenied => "permission_denied",
            Self::ServiceUnavailable => "service_unavailable",
            Self::UpstreamStatus => "upstream_status",
            Self::UpstreamConnect => "upstream_connect",
            Self::UpstreamInvalidResponse => "upstream_invalid_response",
            Self::SharedQuotaExhausted => "shared_quota_exhausted",
            Self::RecordingUnavailable => "recording_unavailable",
            Self::Internal => "internal",
        }
    }
}

/// 管理 API 错误响应体。
///
/// `code` 保留字符串而不是枚举：服务端新增错误码时，旧客户端仍能解析并回退到通用文案。
/// 服务端构造时使用 [`ProtocolErrorBody::new`]，保证写入的是 [`ProtocolErrorCode`] 的稳定取值。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolErrorBody {
    pub code: String,
    pub message: String,
}

impl ProtocolErrorBody {
    pub fn new(code: ProtocolErrorCode, message: impl Into<String>) -> Self {
        Self {
            code: code.as_str().to_owned(),
            message: message.into(),
        }
    }
}

/// 管理 API 的统一错误响应：`{"error": {"code": "...", "message": "..."}}`。
///
/// `message` 只给开发排障，单条消息只用一种语言，不写面向用户的文案；
/// 面向用户的文案由客户端按 `code` 决定。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolErrorResponse {
    pub error: ProtocolErrorBody,
}

impl ProtocolErrorResponse {
    pub fn new(code: ProtocolErrorCode, message: impl Into<String>) -> Self {
        Self {
            error: ProtocolErrorBody::new(code, message),
        }
    }
}
