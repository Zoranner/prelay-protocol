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
    InvalidCredential,
    NotFound,
    ValidationFailed,
    Internal,
}

impl ProtocolErrorCode {
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
            Self::InvalidCredential => "invalid_credential",
            Self::NotFound => "not_found",
            Self::ValidationFailed => "validation_failed",
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
/// `message` 是面向诊断的补充信息，语言不作承诺；面向用户的文案由客户端按 `code` 决定。
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
