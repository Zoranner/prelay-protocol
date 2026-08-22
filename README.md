# prelay-protocol

`prelay-protocol` 定义 Prelay 服务端与桌面客户端之间共享的管理 API 数据传输对象和稳定错误码。它不包含 HTTP 路由、身份认证、数据库、凭据存储、供应商适配或协议桥接实现。

crate 元数据版本为 `0.1.0`，采用 Rust 2021 edition。crate 包名为 `prelay-protocol`，Rust 代码中的导入名为 `prelay_protocol`。

## 依赖

`prelay-server` 和 `prelay-client` 都将本仓作为各自 `crates/protocol` 目录的 Git submodule。两个父仓的 Rust manifest 使用相同的相对路径依赖：

```toml
prelay-protocol = { path = "../crates/protocol" }
```

上例分别适用于 `prelay-server/server/Cargo.toml` 和 `prelay-client/src-tauri/Cargo.toml`；二者到各自 `crates/protocol` submodule 的相对路径均为 `../crates/protocol`。父仓只通过各自的 submodule 引用本 crate。

## 协议材料

`docs/protocol/` 保存 Bruno 请求集合与无密钥环境模板。模板只能包含占位符，不得提交设备凭据、Endpoint Token 或 Provider API Key。

## 验证

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```
