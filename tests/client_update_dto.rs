use prelay_protocol::{ClientUpdateResponse, ClientUpdateTarget, ProtocolErrorCode};

#[test]
fn client_update_target_uses_platform_and_architecture_fields() {
    let target = ClientUpdateTarget {
        platform: "windows".to_string(),
        architecture: "x64".to_string(),
    };

    let value = serde_json::to_value(&target).expect("serialize update target");
    assert_eq!(value["platform"], "windows");
    assert_eq!(value["architecture"], "x64");
    assert_eq!(
        serde_json::from_value::<ClientUpdateTarget>(value).expect("deserialize update target"),
        target
    );
}

#[test]
fn client_update_response_uses_camel_case_fields() {
    let response = ClientUpdateResponse {
        version: "0.2.0".to_string(),
        file_name: "Prelay_0.2.0_x64-setup.exe".to_string(),
        download_path: "/api/client-update/download".to_string(),
    };

    let value = serde_json::to_value(&response).expect("serialize update response");
    assert_eq!(value["version"], "0.2.0");
    assert_eq!(value["fileName"], "Prelay_0.2.0_x64-setup.exe");
    assert_eq!(value["downloadPath"], "/api/client-update/download");
    assert_eq!(
        serde_json::from_value::<ClientUpdateResponse>(value).expect("deserialize update response"),
        response
    );
}

#[test]
fn client_update_unavailable_error_code_is_stable() {
    let code = ProtocolErrorCode::ClientUpdateUnavailable;

    assert_eq!(
        serde_json::to_value(code).expect("serialize error code"),
        "client_update_unavailable"
    );
    assert_eq!(code.as_str(), "client_update_unavailable");
}
