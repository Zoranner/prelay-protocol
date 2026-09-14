use std::fmt::Debug;

use prelay_protocol::{
    endpoints::{EndpointModelResponse, UpdateEndpointRequest},
    stats::{
        ActivitySummary, LeaderboardMetric, ModelStatsSummary, ProviderStatsSummary, StatsOverview,
        TokenUsageTimelinePoint, UserLeaderboardEntry,
    },
};
use prelay_protocol::{
    CatalogImageGenerationModelResponse, CatalogLanguageModelResponse, CatalogProviderResponse,
    CreateEndpointRequest, CreateIdentityRequest, CreateIdentityResponse, CreateProviderRequest,
    EndpointModelInput, EndpointResponse, ProtocolErrorCode, ProviderAuthScheme,
    ProviderCapabilityOverrides, ProviderCatalogResponse, ProviderListItemResponse,
    ProviderOperationResponse, ProviderProtocol, ProviderProtocolBaseUrl, ProviderProtocolBaseUrls,
    ProviderResponse, ProviderSharingResponse, ProviderUsageResponse, ProviderUsageUser,
    ProviderVisibility, RotateCredentialRequest, RotateCredentialResponse,
    TestProviderProtocolRequest, UpdateProviderRequest, UpdateProviderSharingRequest,
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
fn provider_catalog_dtos_separate_model_categories_and_protocols() {
    let catalog = ProviderCatalogResponse {
        language_models: vec![CatalogLanguageModelResponse {
            id: "gpt-5".into(),
            display_name: "GPT-5".into(),
            description: Some("Reasoning model".into()),
            reasoning_efforts: Some(vec!["low".into(), "high".into()]),
            default_reasoning_effort: Some("low".into()),
            context_window: Some(128_000),
            max_context_window: Some(256_000),
            effective_context_window_percent: Some(100),
            input_modalities: Some(vec!["text".into()]),
            supports_parallel_tool_calls: Some(true),
            supports_reasoning_summaries: Some(true),
            supports_image_detail_original: None,
            support_verbosity: Some(true),
            default_verbosity: Some("medium".into()),
            apply_patch_tool_type: None,
            web_search_tool_type: None,
            truncation_policy: None,
            reasoning_summary_format: None,
            default_reasoning_summary: None,
            shell_type: None,
            visibility: Some("public".into()),
            supported_in_api: Some(true),
            priority: Some(1),
            base_instructions: None,
            experimental_supported_tools: None,
            minimal_client_version: None,
        }],
        image_generation_models: vec![CatalogImageGenerationModelResponse {
            id: "gpt-image-1".into(),
            display_name: "GPT Image 1".into(),
            description: None,
            input_modalities: Some(vec!["text".into()]),
            output_modalities: Some(vec!["image".into()]),
            sizes: None,
            quality_options: None,
            background_options: None,
            output_formats: None,
            supports_editing: None,
            supports_mask: None,
            supports_reference_images: None,
            visibility: None,
            supported_in_api: None,
            priority: None,
        }],
        providers: vec![CatalogProviderResponse {
            id: "gotoken".into(),
            name: "GoToken 套餐".into(),
            auth_scheme: ProviderAuthScheme::Bearer,
            base_url: "https://gotoken.cc".into(),
            protocols: vec![
                ProviderProtocol::ChatCompletions,
                ProviderProtocol::Responses,
                ProviderProtocol::AnthropicMessages,
                ProviderProtocol::ImagesGenerations,
            ],
            protocol_base_urls: vec![ProviderProtocolBaseUrl {
                protocol: ProviderProtocol::ImagesGenerations,
                base_url: "https://gotoken.cc/v1".into(),
            }],
            language_models: vec![],
            image_generation_models: vec!["gpt-image-1".into()],
        }],
    };

    assert_json_round_trip(catalog.clone());
    let json = serde_json::to_value(catalog).unwrap();
    assert_eq!(json["language_models"][0]["id"], "gpt-5");
    assert_eq!(json["language_models"][0]["display_name"], "GPT-5");
    assert_eq!(
        json["language_models"][0]["reasoning_efforts"],
        serde_json::json!(["low", "high"])
    );
    assert_eq!(json["image_generation_models"][0]["id"], "gpt-image-1");
    assert_eq!(
        json["image_generation_models"][0]["display_name"],
        "GPT Image 1"
    );
    assert_eq!(
        json["image_generation_models"][0]["output_modalities"],
        serde_json::json!(["image"])
    );
    assert_eq!(
        json["providers"][0]["protocols"],
        serde_json::json!([
            "chat_completions",
            "responses",
            "anthropic_messages",
            "images_generations",
        ])
    );
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
        disabled_models: None,
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
        }],
    };
    let endpoint_update = UpdateEndpointRequest {
        name: Some("OpenAI tools production".into()),
        protocol: Some("responses".into()),
        models: Some(vec![EndpointModelInput {
            provider_id: "provider-a".into(),
            upstream_model: "deepseek-reasoner".into(),
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
        disabled_models: Vec::new(),
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
            display_name: "Assistant".into(),
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
        model_requested_display_name: Some("Assistant".into()),
        model_upstream: Some("deepseek-chat".into()),
        model_upstream_display_name: Some("DeepSeek Chat".into()),
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
    let activity_json = serde_json::to_value(ActivitySummary {
        id: "activity-a".into(),
        created_at: "2026-08-13T00:00:00Z".into(),
        protocol_in: None,
        protocol_upstream: None,
        endpoint_name: None,
        provider_name: None,
        model_requested: Some("assistant".into()),
        model_requested_display_name: Some("Assistant".into()),
        model_upstream: Some("deepseek-chat".into()),
        model_upstream_display_name: Some("DeepSeek Chat".into()),
        status: "success".into(),
        http_status: Some(200),
        error_code: None,
        error_message: None,
        input_tokens: None,
        output_tokens: None,
        is_streaming: None,
        first_token_ms: None,
        cache_read_tokens: None,
        cache_write_tokens: None,
        latency_ms: None,
        upstream_request_id: None,
    })
    .unwrap();
    assert_eq!(activity_json["model_requested"], "assistant");
    assert_eq!(activity_json["model_requested_display_name"], "Assistant");
    assert_eq!(activity_json["model_upstream"], "deepseek-chat");
    assert_eq!(
        activity_json["model_upstream_display_name"],
        "DeepSeek Chat"
    );
    let model_stats = ModelStatsSummary {
        model_requested: Some("assistant".into()),
        model_requested_display_name: Some("Assistant".into()),
        total_requests: 12,
        successful_requests: 10,
        failed_requests: 2,
        input_tokens: 123,
        output_tokens: 456,
        average_latency_ms: Some(789.0),
    };
    assert_json_round_trip(model_stats.clone());
    let model_stats_json = serde_json::to_value(model_stats).unwrap();
    assert_eq!(model_stats_json["model_requested"], "assistant");
    assert_eq!(
        model_stats_json["model_requested_display_name"],
        "Assistant"
    );
    assert!(model_stats_json.get("estimated_cost").is_none());
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

#[test]
fn endpoint_model_input_rejects_custom_public_model_name() {
    let custom_name = serde_json::json!({
        "provider_id": "provider-a",
        "upstream_model": "upstream-model",
        "modelName": "custom-public-name"
    });
    assert!(serde_json::from_value::<EndpointModelInput>(custom_name).is_err());

    let input = EndpointModelInput {
        provider_id: "provider-a".into(),
        upstream_model: "upstream-model".into(),
    };
    assert_json_round_trip(input);
}

#[test]
fn provider_visibility_uses_stable_values() {
    assert_eq!(
        serde_json::to_value(ProviderVisibility::Private).unwrap(),
        "private"
    );
    assert_eq!(
        serde_json::to_value(ProviderVisibility::Selected).unwrap(),
        "selected"
    );
    assert_eq!(
        serde_json::to_value(ProviderVisibility::All).unwrap(),
        "all"
    );
    assert_eq!(
        serde_json::from_value::<ProviderVisibility>("selected".into()).unwrap(),
        ProviderVisibility::Selected
    );
    assert!(serde_json::from_value::<ProviderVisibility>("invalid".into()).is_err());
}

#[test]
fn provider_sharing_dtos_round_trip_selected_identities() {
    let request = UpdateProviderSharingRequest {
        visibility: ProviderVisibility::Selected,
        identity_ids: vec!["identity-b".into(), "identity-c".into()],
    };
    let response = ProviderSharingResponse {
        visibility: ProviderVisibility::Selected,
        selected_identity_ids: vec!["identity-b".into(), "identity-c".into()],
        can_manage: true,
    };

    assert_json_round_trip(request.clone());
    assert_json_round_trip(response.clone());
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        serde_json::json!({
            "visibility": "selected",
            "identity_ids": ["identity-b", "identity-c"]
        })
    );
    assert_eq!(
        serde_json::to_value(response).unwrap(),
        serde_json::json!({
            "visibility": "selected",
            "selected_identity_ids": ["identity-b", "identity-c"],
            "can_manage": true
        })
    );
}

#[test]
fn provider_list_response_has_flat_owner_metadata_without_api_key() {
    let provider = ProviderListItemResponse {
        id: "provider-a".into(),
        name: "DeepSeek".into(),
        provider_type: "openai_compatible".into(),
        base_url: "https://api.deepseek.com".into(),
        capabilities: capabilities(),
        upstream_protocols: vec!["openai".into()],
        disabled_models: Vec::new(),
        owner_identity_id: "identity-a".into(),
        owner_display_name: "研发一组".into(),
        visibility: ProviderVisibility::All,
        selected_identity_ids: Vec::new(),
        can_manage: false,
        created_at: "2026-08-13T00:00:00Z".into(),
    };

    assert_json_round_trip(provider.clone());
    let json = serde_json::to_value(provider).unwrap();
    assert_eq!(json["owner_identity_id"], "identity-a");
    assert_eq!(json["owner_display_name"], "研发一组");
    assert!(json.get("owner").is_none());
    assert_eq!(json["visibility"], "all");
    assert_eq!(json["can_manage"], false);
    assert_eq!(json["selected_identity_ids"], serde_json::json!([]));
    assert!(json.get("api_key").is_none());
    assert!(json.get("api_key_masked").is_none());
}

#[test]
fn provider_usage_response_contains_total_and_user_counters() {
    let usage = ProviderUsageResponse {
        total_requests: 12,
        input_tokens: 123,
        output_tokens: 456,
        total_tokens: 579,
        latest_used_at: Some("2026-09-08T00:00:00Z".into()),
        users: vec![ProviderUsageUser {
            identity_id: "identity-b".into(),
            display_name: "研发二组".into(),
            request_count: 4,
            input_tokens: 40,
            output_tokens: 50,
            total_tokens: 90,
            latest_used_at: Some("2026-09-07T00:00:00Z".into()),
        }],
    };

    assert_json_round_trip(usage.clone());
    let json = serde_json::to_value(usage).unwrap();
    assert_eq!(json["total_requests"], 12);
    assert_eq!(json["total_tokens"], 579);
    assert_eq!(json["users"][0]["display_name"], "研发二组");
    assert!(json.get("api_key").is_none());
    assert!(json.get("credential").is_none());
    assert!(json.get("endpoint_token").is_none());
}

#[test]
fn identity_directory_entry_contains_only_display_fields() {
    let entry = prelay_protocol::IdentityDirectoryEntry {
        identity_id: "identity-b".into(),
        display_name: "研发二组".into(),
    };

    assert_json_round_trip(entry.clone());
    let json = serde_json::to_value(entry).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "identity_id": "identity-b",
            "display_name": "研发二组"
        })
    );
}

#[test]
fn provider_sharing_error_codes_are_stable() {
    let codes = [
        (
            ProtocolErrorCode::InvalidProviderSharing,
            "invalid_provider_sharing",
        ),
        (
            ProtocolErrorCode::ProviderSharingNotAllowed,
            "provider_sharing_not_allowed",
        ),
        (
            ProtocolErrorCode::ProviderNotVisible,
            "provider_not_visible",
        ),
        (ProtocolErrorCode::ProviderNotUsable, "provider_not_usable"),
        (
            ProtocolErrorCode::ProviderRouteUnavailable,
            "provider_route_unavailable",
        ),
    ];

    for (code, expected) in codes {
        assert_eq!(code.as_str(), expected);
        assert_eq!(serde_json::to_value(code).unwrap(), expected);
        assert_eq!(
            serde_json::from_value::<ProtocolErrorCode>(expected.into()).unwrap(),
            code
        );
    }
}
