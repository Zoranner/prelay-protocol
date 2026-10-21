use std::collections::BTreeMap;

use prelay_protocol::{
    CatalogKind, ExtensionArtifact, ExtensionArtifactSummary, ExtensionFile,
    ExtensionInstallBundle, ExtensionKind, ExtensionMcpManifest, ExtensionMcpTransport,
    ExtensionSummary, ExtensionVersion, ProtocolErrorCode,
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
        catalog_kind: Some(CatalogKind::Rule),
        artifacts: Vec::new(),
    };
    let bundle = ExtensionInstallBundle {
        name: summary.name.clone(),
        version,
        artifacts: vec![ExtensionArtifact {
            kind: ExtensionKind::Rule,
            files: vec![ExtensionFile {
                path: "AGENTS.md".to_string(),
                content_base64: "IyBFbmdpbmVlcmluZyBydWxlcw==".to_string(),
            }],
        }],
        skipped: vec!["README.md".to_string()],
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
            },
            "catalogKind": "rule",
            "artifacts": []
        })
    );
    assert_eq!(
        serde_json::to_value(bundle).unwrap(),
        serde_json::json!({
            "name": "engineering-rules",
            "version": {
                "tag": "v1.2.3",
                "commitSha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "updatedAt": "2026-08-28T08:30:00Z"
            },
            "artifacts": [{
                "kind": "rule",
                "files": [{
                    "path": "AGENTS.md",
                    "contentBase64": "IyBFbmdpbmVlcmluZyBydWxlcw=="
                }]
            }],
            "skipped": ["README.md"],
            "kind": "rule",
            "files": [{
                "path": "AGENTS.md",
                "contentBase64": "IyBFbmdpbmVlcmluZyBydWxlcw=="
            }]
        })
    );
    assert!(serde_json::from_value::<ExtensionFile>(serde_json::json!({
        "path": "AGENTS.md",
        "content": "# Engineering rules"
    }))
    .is_err());
}

#[test]
fn install_bundle_accepts_the_legacy_single_artifact_shape() {
    let bundle: ExtensionInstallBundle = serde_json::from_value(serde_json::json!({
        "name": "engineering-skills",
        "kind": "skill",
        "version": {
            "tag": "v1.2.3",
            "commitSha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "updatedAt": "2026-08-28T08:30:00Z"
        },
        "files": [{
            "path": "skills/review/SKILL.md",
            "contentBase64": "IyBSZXZpZXc="
        }]
    }))
    .unwrap();

    assert_eq!(bundle.artifacts.len(), 1);
    assert_eq!(bundle.artifacts[0].kind, ExtensionKind::Skill);
    assert_eq!(bundle.artifacts[0].files.len(), 1);
    assert_eq!(bundle.artifacts[0].files[0].path, "skills/review/SKILL.md");
    assert!(bundle.skipped.is_empty());
}

#[test]
fn install_bundle_round_trips_multiple_artifacts() {
    let bundle = ExtensionInstallBundle {
        name: "superpowers".to_string(),
        version: ExtensionVersion {
            tag: "v6.3.1".to_string(),
            commit_sha: "b".repeat(40),
            updated_at: "2026-10-21T08:30:00Z".to_string(),
        },
        artifacts: vec![
            ExtensionArtifact {
                kind: ExtensionKind::Skill,
                files: vec![ExtensionFile {
                    path: "skills/using-superpowers/SKILL.md".to_string(),
                    content_base64: "IyBVc2luZyBzdXBlcnBvd2Vycw==".to_string(),
                }],
            },
            ExtensionArtifact {
                kind: ExtensionKind::Hook,
                files: vec![ExtensionFile {
                    path: "hooks/hooks.json".to_string(),
                    content_base64: "e30=".to_string(),
                }],
            },
        ],
        skipped: vec!["README.md".to_string(), "LICENSE".to_string()],
    };

    let value = serde_json::to_value(&bundle).unwrap();
    assert_eq!(
        value["artifacts"][0]["kind"],
        serde_json::Value::String("skill".to_string())
    );
    assert_eq!(
        value["artifacts"][1]["kind"],
        serde_json::Value::String("hook".to_string())
    );
    assert_eq!(
        serde_json::from_value::<ExtensionInstallBundle>(value).unwrap(),
        bundle
    );
}

#[test]
fn unknown_artifact_kinds_remain_readable_for_newer_servers() {
    let kind: ExtensionKind = serde_json::from_value(serde_json::json!("future")).unwrap();
    assert_eq!(kind, ExtensionKind::Unknown);

    let summary: ExtensionSummary = serde_json::from_value(serde_json::json!({
        "name": "future-extension",
        "repository": "https://git.example.test/agents/future-extension",
        "latest": {
            "tag": "v2.0.0",
            "commitSha": "cccccccccccccccccccccccccccccccccccccccc",
            "updatedAt": "2026-10-21T08:30:00Z"
        },
        "catalogKind": "plugin",
        "artifacts": [{
            "kind": "future",
            "fileCount": 2
        }]
    }))
    .unwrap();

    assert_eq!(summary.catalog_kind, Some(CatalogKind::Plugin));
    assert_eq!(
        summary.artifacts,
        vec![ExtensionArtifactSummary {
            kind: ExtensionKind::Unknown,
            file_count: 2,
        }]
    );
}

#[test]
fn extension_summary_defaults_fields_missing_from_old_servers() {
    let summary: ExtensionSummary = serde_json::from_value(serde_json::json!({
        "name": "engineering-rules",
        "repository": "https://git.example.test/agents/engineering-rules",
        "latest": {
            "tag": "v1.0.0",
            "commitSha": "dddddddddddddddddddddddddddddddddddddddd",
            "updatedAt": "2026-08-28T08:30:00Z"
        }
    }))
    .unwrap();

    assert_eq!(summary.catalog_kind, None);
    assert!(summary.artifacts.is_empty());
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
    assert_eq!(
        ProtocolErrorCode::ExtensionContentInvalid.as_str(),
        "extension_content_invalid"
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
