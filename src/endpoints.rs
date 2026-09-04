use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EndpointModelInput {
    pub provider_id: String,
    pub upstream_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateEndpointRequest {
    pub name: String,
    pub protocol: Option<String>,
    pub models: Vec<EndpointModelInput>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateEndpointRequest {
    pub name: Option<String>,
    pub protocol: Option<String>,
    pub models: Option<Vec<EndpointModelInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EndpointModelResponse {
    pub id: String,
    pub endpoint_id: String,
    pub model_name: String,
    pub display_name: String,
    pub provider_id: String,
    pub upstream_model: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EndpointResponse {
    pub id: String,
    pub name: String,
    pub protocol: String,
    pub token: String,
    pub models: Vec<EndpointModelResponse>,
    pub created_at: String,
}
