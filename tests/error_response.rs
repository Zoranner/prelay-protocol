use prelay_protocol::{ProtocolErrorBody, ProtocolErrorCode, ProtocolErrorResponse};
use serde_json::json;

#[test]
fn error_response_serializes_the_stable_code() {
    let response = ProtocolErrorResponse::new(
        ProtocolErrorCode::ProviderNotVisible,
        "provider is not visible to identity",
    );

    assert_eq!(
        serde_json::to_value(response).expect("serialize error response"),
        json!({
            "error": {
                "code": "provider_not_visible",
                "message": "provider is not visible to identity"
            }
        })
    );
}

#[test]
fn error_body_reads_an_unknown_code_without_failing() {
    let body: ProtocolErrorBody = serde_json::from_value(json!({
        "code": "code_from_a_newer_server",
        "message": "future failure"
    }))
    .expect("unknown codes must stay parseable");

    assert_eq!(body.code, "code_from_a_newer_server");
    assert_eq!(body.message, "future failure");
}

#[test]
fn error_response_round_trips_every_known_code() {
    for code in [
        ProtocolErrorCode::IdentityAlreadyRegistered,
        ProtocolErrorCode::ClientUpdateUnavailable,
        ProtocolErrorCode::ExtensionCatalogUnavailable,
        ProtocolErrorCode::ExtensionContentInvalid,
        ProtocolErrorCode::ExtensionNotFound,
        ProtocolErrorCode::ExtensionVersionNotFound,
        ProtocolErrorCode::ExtensionInstallUnsupported,
        ProtocolErrorCode::InvalidProviderSharing,
        ProtocolErrorCode::ProviderSharingNotAllowed,
        ProtocolErrorCode::ProviderNotVisible,
        ProtocolErrorCode::ProviderNotUsable,
        ProtocolErrorCode::ProviderRouteUnavailable,
        ProtocolErrorCode::InvalidCredential,
        ProtocolErrorCode::NotFound,
        ProtocolErrorCode::ValidationFailed,
        ProtocolErrorCode::Internal,
    ] {
        let response = ProtocolErrorResponse::new(code, "detail");
        let value = serde_json::to_value(&response).expect("serialize error response");
        assert_eq!(
            value["error"]["code"].as_str(),
            Some(code.as_str()),
            "{}",
            code.as_str()
        );
        let decoded: ProtocolErrorResponse =
            serde_json::from_value(value).expect("deserialize error response");
        assert_eq!(decoded, response);
    }
}
