use std::collections::BTreeMap;

use prelay_protocol::{
    validate_mcp_manifest, ExtensionMcpManifest, ExtensionMcpTransport, McpManifestError,
};

fn stdio(name: &str, command: &[&str]) -> ExtensionMcpManifest {
    ExtensionMcpManifest {
        name: name.to_string(),
        transport: ExtensionMcpTransport::Stdio {
            command: command
                .iter()
                .map(|argument| argument.to_string())
                .collect(),
            cwd: None,
            environment: BTreeMap::new(),
            enabled: true,
            timeout_ms: None,
        },
    }
}

fn http(name: &str, url: &str) -> ExtensionMcpManifest {
    ExtensionMcpManifest {
        name: name.to_string(),
        transport: ExtensionMcpTransport::Http {
            url: url.to_string(),
            headers: BTreeMap::new(),
            enabled: true,
            timeout_ms: None,
        },
    }
}

#[test]
fn accepts_safe_stdio_and_http_manifests() {
    let mut valid_stdio = stdio("filesystem", &["uvx", "mcp-server-filesystem"]);
    if let ExtensionMcpTransport::Stdio { environment, .. } = &mut valid_stdio.transport {
        environment.insert("GITHUB_TOKEN".to_string(), "GITHUB_TOKEN".to_string());
    }
    assert_eq!(validate_mcp_manifest(&valid_stdio), Ok(()));

    let mut valid_http = http("remote", "https://mcp.example.test/mcp");
    if let ExtensionMcpTransport::Http { headers, .. } = &mut valid_http.transport {
        headers.insert("Authorization".to_string(), "API_KEY".to_string());
    }
    assert_eq!(validate_mcp_manifest(&valid_http), Ok(()));

    assert_eq!(
        validate_mcp_manifest(&stdio("filesystem", &["uvx", "--tokenizer=cl100k_base"])),
        Ok(())
    );
    assert_eq!(
        validate_mcp_manifest(&stdio(
            "filesystem",
            &["uvx", "https://docs.example.test/guide"]
        )),
        Ok(())
    );
}

#[test]
fn rejects_unusable_server_names() {
    assert_eq!(
        validate_mcp_manifest(&stdio("filesystem server", &["uvx"])),
        Err(McpManifestError::ServerName)
    );
    assert_eq!(
        validate_mcp_manifest(&stdio("workspace", &["uvx"])),
        Err(McpManifestError::ServerName)
    );
}

#[test]
fn rejects_stdio_manifests_that_cannot_be_written_safely() {
    let empty_command = stdio("filesystem", &[]);
    assert_eq!(
        validate_mcp_manifest(&empty_command),
        Err(McpManifestError::StdioTransport)
    );

    let mut working_directory = stdio("filesystem", &["uvx"]);
    if let ExtensionMcpTransport::Stdio { cwd, .. } = &mut working_directory.transport {
        *cwd = Some("C:/workspace".to_string());
    }
    assert_eq!(
        validate_mcp_manifest(&working_directory),
        Err(McpManifestError::StdioTransport)
    );

    let mut disabled = stdio("filesystem", &["uvx"]);
    if let ExtensionMcpTransport::Stdio { enabled, .. } = &mut disabled.transport {
        *enabled = false;
    }
    assert_eq!(
        validate_mcp_manifest(&disabled),
        Err(McpManifestError::StdioTransport)
    );

    let mut remapped = stdio("filesystem", &["uvx"]);
    if let ExtensionMcpTransport::Stdio { environment, .. } = &mut remapped.transport {
        environment.insert("GITHUB_TOKEN".to_string(), "PRELAY_TOKEN".to_string());
    }
    assert_eq!(
        validate_mcp_manifest(&remapped),
        Err(McpManifestError::StdioTransport)
    );

    let mut plaintext_environment = stdio("filesystem", &["uvx"]);
    if let ExtensionMcpTransport::Stdio { environment, .. } = &mut plaintext_environment.transport {
        environment.insert("GITHUB_TOKEN".to_string(), "plain-text-secret".to_string());
    }
    assert_eq!(
        validate_mcp_manifest(&plaintext_environment),
        Err(McpManifestError::StdioTransport)
    );
}

#[test]
fn rejects_plaintext_credentials_in_command_arguments() {
    for command in [
        vec!["uvx", "mcp-server-filesystem", "--api-key", "secret"],
        vec!["uvx", "mcp-server-filesystem", "--access-key", "secret"],
        vec!["uvx", "mcp-server-filesystem", "--client-secret=secret"],
        vec![
            "uvx",
            "mcp-remote",
            "--header",
            "Authorization: Bearer secret",
        ],
        vec![
            "uvx",
            "mcp-remote",
            "https://user:secret@mcp.example.test/mcp",
        ],
        vec![
            "uvx",
            "mcp-remote",
            "https://mcp.example.test/mcp?access_token=secret",
        ],
    ] {
        assert_eq!(
            validate_mcp_manifest(&stdio("filesystem", &command)),
            Err(McpManifestError::StdioTransport),
            "accepted {command:?}"
        );
    }
}

#[test]
fn rejects_http_manifests_that_are_not_safe() {
    for url in [
        "file:///C:/server",
        "https://mcp.example.test?token=secret",
        "https://mcp.example.test?transport=stream",
        "https://user:secret@mcp.example.test/mcp",
        "https://mcp.example.test/mcp#fragment",
    ] {
        assert_eq!(
            validate_mcp_manifest(&http("remote", url)),
            Err(McpManifestError::HttpTransport),
            "accepted {url}"
        );
    }

    let mut disabled = http("remote", "https://mcp.example.test/mcp");
    if let ExtensionMcpTransport::Http { enabled, .. } = &mut disabled.transport {
        *enabled = false;
    }
    assert_eq!(
        validate_mcp_manifest(&disabled),
        Err(McpManifestError::HttpTransport)
    );

    let mut plaintext_header = http("remote", "https://mcp.example.test/mcp");
    if let ExtensionMcpTransport::Http { headers, .. } = &mut plaintext_header.transport {
        headers.insert(
            "Authorization".to_string(),
            "Bearer plain-text-secret".to_string(),
        );
    }
    assert_eq!(
        validate_mcp_manifest(&plaintext_header),
        Err(McpManifestError::HttpTransport)
    );
}
