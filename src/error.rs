use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolErrorCode {
    IdentityAlreadyRegistered,
    ClientUpdateUnavailable,
    ExtensionCatalogUnavailable,
    ExtensionNotFound,
    ExtensionVersionNotFound,
    ExtensionInstallUnsupported,
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
            Self::ExtensionNotFound => "extension_not_found",
            Self::ExtensionVersionNotFound => "extension_version_not_found",
            Self::ExtensionInstallUnsupported => "extension_install_unsupported",
            Self::InvalidCredential => "invalid_credential",
            Self::NotFound => "not_found",
            Self::ValidationFailed => "validation_failed",
            Self::Internal => "internal",
        }
    }
}
