use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

mod manifest;

pub use manifest::{validate_mcp_manifest, McpManifestError};

/// 扩展制品类型。
///
/// [`ExtensionKind::Unknown`] 只用于客户端解析未来服务端新增的制品类型，
/// 服务端不得把它作为目录或安装包中的有效类型。
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionKind {
    Rule,
    Skill,
    Mcp,
    Hook,
    Unknown,
}

impl<'de> Deserialize<'de> for ExtensionKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "rule" => Self::Rule,
            "skill" => Self::Skill,
            "mcp" => Self::Mcp,
            "hook" => Self::Hook,
            _ => Self::Unknown,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CatalogKind {
    Rule,
    Plugin,
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
    /// 旧服务端不返回该字段；客户端应按请求的过渡接口推断单制品类型。
    #[serde(default)]
    pub catalog_kind: Option<CatalogKind>,
    #[serde(default)]
    pub artifacts: Vec<ExtensionArtifactSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionFile {
    pub path: String,
    pub content_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionArtifact {
    pub kind: ExtensionKind,
    pub files: Vec<ExtensionFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionArtifactSummary {
    pub kind: ExtensionKind,
    pub file_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionInstallBundle {
    pub name: String,
    pub version: ExtensionVersion,
    pub artifacts: Vec<ExtensionArtifact>,
    pub skipped: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExtensionInstallBundleOutput<'a> {
    name: &'a str,
    version: &'a ExtensionVersion,
    artifacts: &'a [ExtensionArtifact],
    skipped: &'a [String],
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<ExtensionKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<&'a [ExtensionFile]>,
}

impl Serialize for ExtensionInstallBundle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let legacy = match self.artifacts.as_slice() {
            [artifact]
                if matches!(
                    artifact.kind,
                    ExtensionKind::Rule | ExtensionKind::Skill | ExtensionKind::Mcp
                ) =>
            {
                Some(artifact)
            }
            _ => None,
        };
        ExtensionInstallBundleOutput {
            name: &self.name,
            version: &self.version,
            artifacts: &self.artifacts,
            skipped: &self.skipped,
            kind: legacy.map(|artifact| artifact.kind),
            files: legacy.map(|artifact| artifact.files.as_slice()),
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExtensionInstallBundleWire {
    name: String,
    version: ExtensionVersion,
    #[serde(default)]
    artifacts: Vec<ExtensionArtifact>,
    #[serde(default)]
    skipped: Vec<String>,
    #[serde(default)]
    kind: Option<ExtensionKind>,
    #[serde(default)]
    files: Vec<ExtensionFile>,
}

impl<'de> Deserialize<'de> for ExtensionInstallBundle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = ExtensionInstallBundleWire::deserialize(deserializer)?;
        let artifacts = if wire.artifacts.is_empty() {
            wire.kind
                .map(|kind| ExtensionArtifact {
                    kind,
                    files: wire.files,
                })
                .into_iter()
                .collect()
        } else {
            wire.artifacts
        };
        Ok(Self {
            name: wire.name,
            version: wire.version,
            artifacts,
            skipped: wire.skipped,
        })
    }
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
