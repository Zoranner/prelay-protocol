use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

mod manifest;

pub use manifest::{validate_mcp_manifest, McpManifestError};

/// Hook 定义文件在扩展包内的固定相对路径。
///
/// `hooks/` 下的其他文件都是支持文件，不参与 Hook 定义解析。
pub const HOOKS_MANIFEST_PATH: &str = "hooks/hooks.json";

/// Hook 命令中引用客户端受管安装根的占位符。
///
/// 客户端在写入宿主 Hook 配置前只替换该占位符，其他 `${...}` 保持不变。
pub const HOOK_ROOT_PLACEHOLDER: &str = "${PRELAY_HOOK_ROOT}";

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

/// 扩展包中的单个文件。
///
/// `content_base64` 始终承载原始字节。Hook 支持文件可以是任意二进制内容，
/// 服务端和客户端不得按 UTF-8 或 JSON 重新解释。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionFile {
    /// 扩展包内相对路径。Hook 支持文件按这里的路径原样安装。
    pub path: String,
    /// 文件原始字节的 Base64 编码；支持文件可以不是 UTF-8，也可以是二进制。
    pub content_base64: String,
}

/// 一个安装制品及其全部文件。
///
/// Hook 制品同时包含 `hooks/hooks.json` 定义文件和 `hooks/` 下的支持文件。
/// 只有定义文件需要解析；其他文件按字节安装，不要求是 UTF-8 或 JSON。
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
