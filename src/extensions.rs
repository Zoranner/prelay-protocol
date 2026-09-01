use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionKind {
    Rule,
    Skill,
    Mcp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionVersion {
    pub tag: String,
    pub commit_sha: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionSummary {
    pub name: String,
    pub repository: String,
    pub latest: ExtensionVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionFile {
    pub path: String,
    pub content_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionInstallBundle {
    pub name: String,
    pub kind: ExtensionKind,
    pub version: ExtensionVersion,
    pub files: Vec<ExtensionFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionMcpManifest {
    pub name: String,
    pub transport: ExtensionMcpTransport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    tag = "type",
    rename_all = "lowercase",
    rename_all_fields = "camelCase"
)]
pub enum ExtensionMcpTransport {
    Stdio {
        command: Vec<String>,
        cwd: Option<String>,
        environment: BTreeMap<String, String>,
        enabled: bool,
        timeout_ms: Option<u64>,
    },
    Http {
        url: String,
        headers: BTreeMap<String, String>,
        enabled: bool,
        timeout_ms: Option<u64>,
    },
}
