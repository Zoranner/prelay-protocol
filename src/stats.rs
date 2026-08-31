use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatsOverview {
    pub total_requests: i64,
    pub successful_requests: i64,
    pub failed_requests: i64,
    pub input_tokens: i64,
    pub total_input_tokens: i64,
    pub output_tokens: i64,
    pub cache_read_tokens: i64,
    pub cache_write_tokens: i64,
    pub average_latency_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenUsageTimelinePoint {
    pub bucket: String,
    pub input_tokens: i64,
    pub total_input_tokens: i64,
    pub output_tokens: i64,
    pub cache_read_tokens: i64,
    pub cache_write_tokens: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActivitySummary {
    pub id: String,
    pub created_at: String,
    pub protocol_in: Option<String>,
    pub protocol_upstream: Option<String>,
    pub endpoint_name: Option<String>,
    pub provider_name: Option<String>,
    pub model_requested: Option<String>,
    pub model_upstream: Option<String>,
    pub status: String,
    pub http_status: Option<i64>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub is_streaming: Option<bool>,
    pub first_token_ms: Option<i64>,
    pub cache_read_tokens: Option<i64>,
    pub cache_write_tokens: Option<i64>,
    pub latency_ms: Option<i64>,
    pub upstream_request_id: Option<String>,
    pub metadata_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelStatsSummary {
    pub model_requested: Option<String>,
    pub total_requests: i64,
    pub successful_requests: i64,
    pub failed_requests: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub estimated_cost: Option<f64>,
    pub average_latency_ms: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProviderStatsSummary {
    pub provider_id: Option<String>,
    pub provider_name: Option<String>,
    pub total_requests: i64,
    pub successful_requests: i64,
    pub failed_requests: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub estimated_cost: Option<f64>,
    pub average_latency_ms: Option<f64>,
    pub average_first_token_ms: Option<f64>,
}
