use std::fmt::Debug;

use prelay_protocol::{
    endpoints::{EndpointModelResponse, UpdateEndpointRequest},
    providers::ProviderModelResponse,
    stats::{
        ActivitySummary, LeaderboardMetric, ModelStatsSummary, ProviderStatsSummary, StatsOverview,
        TokenUsageTimelinePoint, UserLeaderboardEntry,
    },
};
use prelay_protocol::{
    CreateEndpointRequest, CreateIdentityRequest, CreateIdentityResponse, CreateProviderRequest,
    EndpointModelInput, EndpointResponse, ProtocolErrorCode, ProviderCapabilityOverrides,
    ProviderOperationResponse, ProviderProtocolBaseUrls, ProviderResponse, RotateCredentialRequest,
    RotateCredentialResponse, TestProviderProtocolRequest, UpdateProviderRequest,
};
use serde::{de::DeserializeOwned, Serialize};

fn assert_json_round_trip<T>(value: T)
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(serde_json::from_value::<T>(json).unwrap(), value);
}

fn capabilities() -> ProviderCapabilityOverrides {
    ProviderCapabilityOverrides {
        upstream_protocols: Some(vec!["openai".into(), "images_generations".into()]),
        protocol_base_urls: Some(ProviderProtocolBaseUrls {
            openai: Some("https://api.example/v1".into()),
            images_generations: Some("https://images.example/v1".into()),
            ..Default::default()
        }),
        tool_calls: Some(true),
        ..Default::default()
    }
}

#[test]
fn management_requests_round_trip_without_client_identity_id() {
    let register = CreateIdentityRequest {
        machine_id: "machine-a".into(),
        account_sid: "S-1-5-21-100".into(),
        credential: "client-generated-credential".into(),
        display_name: Some("Ada Lovelace".into()),
    };
    let provider = CreateProviderRequest {
        name: "DeepSeek".into(),
        provider_type: "openai_compatible".into(),
        base_url: "https://api.deepseek.com".into(),
        api_key: "sk-test".into(),
        capabilities: Some(capabilities()),
        models: vec!["deepseek-chat".into()],
    };
    let update = UpdateProviderRequest {
        name: Some("DeepSeek Production".into()),
        capabilities: Some(capabilities()),
        ..Default::default()
    };
    let endpoint = CreateEndpointRequest {
        name: "OpenAI tools".into(),
        protocol: Some("openai".into()),
        models: vec![EndpointModelInput {
            provider_id: "provider-a".into(),
            upstream_model: "deepseek-chat".into(),
            model_name: Some("assistant".into()),
        }],
    };
    let endpoint_update = UpdateEndpointRequest {
        name: Some("OpenAI tools production".into()),
        protocol: Some("responses".into()),
        models: Some(vec![EndpointModelInput {
            provider_id: "provider-a".into(),
            upstream_model: "deepseek-reasoner".into(),
            model_name: Some("reasoner".into()),
        }]),
    };
    let empty_endpoint_update = UpdateEndpointRequest::default();

    assert_json_round_trip(register.clone());
    assert_json_round_trip(provider.clone());
    assert_json_round_trip(update.clone());
    assert_json_round_trip(endpoint);
    assert_json_round_trip(endpoint_update);
    assert_json_round_trip(empty_endpoint_update.clone());

    assert!(serde_json::to_value(register)
        .unwrap()
        .get("identity_id")
        .is_none());
    assert!(serde_json::to_value(&provider)
        .unwrap()
        .get("identity_id")
        .is_none());
    assert_eq!(
        serde_json::to_value(provider).unwrap()["capabilities"]["protocol_base_urls"]
            ["images_generations"],
        "https://images.example/v1"
    );
    assert_eq!(
        serde_json::to_value(update).unwrap()["api_key"],
        serde_json::Value::Null
    );
    let empty_endpoint_update_json = serde_json::to_value(empty_endpoint_update).unwrap();
    assert_eq!(empty_endpoint_update_json["name"], serde_json::Value::Null);
    assert_eq!(
        empty_endpoint_update_json["protocol"],
        serde_json::Value::Null
    );
    assert_eq!(
        empty_endpoint_update_json["models"],
        serde_json::Value::Null
    );
    assert_eq!(
        EndpointModelInput::default_model_name("upstream"),
        "upstream"
    );
}

#[test]
fn management_responses_and_stats_round_trip() {
    assert_json_round_trip(CreateIdentityResponse {
        identity_id: "identity-a".into(),
        created: true,
    });
    assert_json_round_trip(RotateCredentialRequest {
        new_credential: "next-client-credential".into(),
    });
    assert_json_round_trip(RotateCredentialResponse { rotated: true });
    assert_json_round_trip(ProviderResponse {
        id: "provider-a".into(),
        name: "DeepSeek".into(),
        provider_type: "openai_compatible".into(),
        base_url: "https://api.deepseek.com".into(),
        api_key: "sk-test".into(),
        api_key_masked: "sk-t...test".into(),
        capabilities: capabilities(),
        upstream_protocols: vec!["openai".into(), "anthropic".into()],
        models: vec![ProviderModelResponse {
            id: "model-a".into(),
            provider_id: "provider-a".into(),
            model_name: "deepseek-chat".into(),
            created_at: "2026-08-13T00:00:00Z".into(),
        }],
        created_at: "2026-08-13T00:00:00Z".into(),
    });
    assert_json_round_trip(EndpointResponse {
        id: "endpoint-a".into(),
        name: "OpenAI tools".into(),
        protocol: "openai".into(),
        token: "endpoint-token".into(),
        models: vec![EndpointModelResponse {
            id: "endpoint-model-a".into(),
            endpoint_id: "endpoint-a".into(),
            model_name: "assistant".into(),
            provider_id: "provider-a".into(),
            upstream_model: "deepseek-chat".into(),
            created_at: "2026-08-13T00:00:00Z".into(),
        }],
        created_at: "2026-08-13T00:00:00Z".into(),
    });
    assert_json_round_trip(StatsOverview {
        total_requests: 12,
        successful_requests: 10,
        failed_requests: 2,
        input_tokens: 123,
        total_input_tokens: 140,
        output_tokens: 456,
        cache_read_tokens: 8,
        cache_write_tokens: 9,
        average_latency_ms: Some(789),
    });
    assert_json_round_trip(TokenUsageTimelinePoint {
        bucket: "2026-08-22".into(),
        input_tokens: 123,
        total_input_tokens: 140,
        output_tokens: 456,
        cache_read_tokens: 78,
        cache_write_tokens: 9,
    });
    assert_json_round_trip(ActivitySummary {
        id: "activity-a".into(),
        created_at: "2026-08-13T00:00:00Z".into(),
        protocol_in: Some("responses".into()),
        protocol_upstream: None,
        endpoint_name: Some("生产接入点".into()),
        provider_name: Some("DeepSeek".into()),
        model_requested: Some("assistant".into()),
        model_upstream: Some("deepseek-chat".into()),
        status: "success".into(),
        http_status: Some(200),
        error_code: None,
        error_message: None,
        input_tokens: Some(123),
        output_tokens: Some(456),
        is_streaming: Some(true),
        first_token_ms: Some(120),
        cache_read_tokens: Some(32),
        cache_write_tokens: Some(16),
        latency_ms: Some(789),
        upstream_request_id: None,
    });
    let model_stats = ModelStatsSummary {
        model_requested: Some("assistant".into()),
        total_requests: 12,
        successful_requests: 10,
        failed_requests: 2,
        input_tokens: 123,
        output_tokens: 456,
        average_latency_ms: Some(789.0),
    };
    assert_json_round_trip(model_stats.clone());
    assert!(serde_json::to_value(model_stats)
        .unwrap()
        .get("estimated_cost")
        .is_none());
    let provider_stats = ProviderStatsSummary {
        provider_id: Some("provider-a".into()),
        provider_name: Some("DeepSeek".into()),
        total_requests: 12,
        successful_requests: 10,
        failed_requests: 2,
        input_tokens: 123,
        output_tokens: 456,
        average_latency_ms: Some(789.0),
        average_first_token_ms: None,
    };
    assert_json_round_trip(provider_stats.clone());
    assert!(serde_json::to_value(provider_stats)
        .unwrap()
        .get("estimated_cost")
        .is_none());
    assert_eq!(
        serde_json::to_value(LeaderboardMetric::TotalTokens).unwrap(),
        "total_tokens"
    );
    assert_json_round_trip(UserLeaderboardEntry {
        rank: 1,
        identity_id: "identity-a".into(),
        display_name: "研发一组".into(),
        activity_count: 12,
        total_tokens: 5_678,
        successful_activities: 10,
        success_rate: 10.0 / 12.0,
    });
}

#[test]
fn identity_credential_dtos_round_trip_without_server_issued_secret() {
    let request = CreateIdentityRequest {
        machine_id: "machine-a".into(),
        account_sid: "S-1-5-21-100".into(),
        credential: "client-generated-credential".into(),
        display_name: None,
    };
    let rotate = RotateCredentialRequest {
        new_credential: "next-client-credential".into(),
    };

    assert_eq!(
        serde_json::to_value(request).unwrap()["credential"],
        "client-generated-credential"
    );
    assert_eq!(
        serde_json::to_value(rotate).unwrap()["new_credential"],
        "next-client-credential"
    );
    assert!(serde_json::to_value(CreateIdentityResponse {
        identity_id: "identity-a".into(),
        created: false,
    })
    .unwrap()
    .get("credential")
    .is_none());
}

#[test]
fn provider_operation_dtos_round_trip() {
    assert_json_round_trip(TestProviderProtocolRequest {
        protocol: "openai".into(),
        model: Some("deepseek-chat".into()),
    });
    assert_json_round_trip(ProviderOperationResponse {
        ok: false,
        protocol: Some("openai".into()),
        latency_ms: Some(42),
        first_token_ms: None,
        error: Some("上游连接失败".into()),
        models: None,
    });
}

#[test]
fn protocol_error_code_uses_stable_snake_case_json() {
    let code = ProtocolErrorCode::IdentityAlreadyRegistered;

    assert_eq!(
        serde_json::to_value(code).unwrap(),
        "identity_already_registered"
    );
    assert_eq!(
        serde_json::from_value::<ProtocolErrorCode>("identity_already_registered".into()).unwrap(),
        code
    );
    assert_eq!(code.as_str(), "identity_already_registered");
}
