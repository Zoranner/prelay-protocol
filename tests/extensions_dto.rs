use std::collections::BTreeMap;

use prelay_protocol::{
    ExtensionFile, ExtensionInstallBundle, ExtensionKind, ExtensionMcpManifest,
    ExtensionMcpTransport, ExtensionSummary, ExtensionVersion, ProtocolErrorCode,
};

#[test]
fn extension_catalog_dtos_use_management_api_field_names() {
    let version = ExtensionVersion {
        tag: "v1.2.3".to_string(),
        commit_sha: "a".repeat(40),
        updated_at: "2026-08-28T08:30:00Z".to_string(),
    };
    let summary = ExtensionSummary {
        name: "engineering-rules".to_string(),
        repository: "https://git.example.test/agents/engineering-rules".to_string(),
        latest: version.clone(),
    };
    let bundle = ExtensionInstallBundle {
        name: summary.name.clone(),
        kind: ExtensionKind::Rule,
        version,
        files: vec![ExtensionFile {
            path: "AGENTS.md".to_string(),
            content: "# Engineering rules".to_string(),
        }],
    };

    assert_eq!(
        serde_json::to_value(summary).unwrap(),
        serde_json::json!({
            "name": "engineering-rules",
            "repository": "https://git.example.test/agents/engineering-rules",
            "latest": {
                "tag": "v1.2.3",
                "commitSha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "updatedAt": "2026-08-28T08:30:00Z"
            }
        })
    );
    assert_eq!(
        serde_json::to_value(bundle).unwrap(),
        serde_json::json!({
            "name": "engineering-rules",
            "kind": "rule",
            "version": {
                "tag": "v1.2.3",
                "commitSha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "updatedAt": "2026-08-28T08:30:00Z"
            },
            "files": [{
                "path": "AGENTS.md",
                "content": "# Engineering rules"
            }]
        })
    );
}

#[test]
fn extension_error_codes_are_stable() {
    assert_eq!(
        ProtocolErrorCode::ExtensionCatalogUnavailable.as_str(),
        "extension_catalog_unavailable"
    );
    assert_eq!(
        ProtocolErrorCode::ExtensionNotFound.as_str(),
        "extension_not_found"
    );
    assert_eq!(
        ProtocolErrorCode::ExtensionVersionNotFound.as_str(),
        "extension_version_not_found"
    );
    assert_eq!(
        ProtocolErrorCode::ExtensionInstallUnsupported.as_str(),
        "extension_install_unsupported"
    );
}

#[test]
fn mcp_manifest_uses_a_tagged_transport() {
    let manifest = ExtensionMcpManifest {
        name: "research".to_string(),
        transport: ExtensionMcpTransport::Stdio {
            command: vec!["prelay-research".to_string(), "--stdio".to_string()],
            cwd: None,
            environment: BTreeMap::from([("MODE".to_string(), "read-only".to_string())]),
            enabled: true,
            timeout_ms: Some(10_000),
        },
    };

    assert_eq!(
        serde_json::to_value(manifest).unwrap(),
        serde_json::json!({
            "name": "research",
            "transport": {
                "type": "stdio",
                "command": ["prelay-research", "--stdio"],
                "cwd": null,
                "environment": { "MODE": "read-only" },
                "enabled": true,
                "timeoutMs": 10_000
            }
        })
    );
}
