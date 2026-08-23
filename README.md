# prelay-protocol

`prelay-protocol` 是 Prelay 管理 API 数据传输对象、稳定错误码和协议材料的唯一来源。它不包含 HTTP 路由、身份认证、数据库、凭据存储、供应商适配或协议桥接实现。

crate 包名为 `prelay-protocol`，Rust 导入名为 `prelay_protocol`，采用 Rust 2021 edition。

## 使用方

`prelay-server` 和 `prelay-client` 都通过各自的 `crates/protocol` Git submodule 引用本仓；管理 DTO 或错误码变更必须先在此处完成，再更新两个父仓的 submodule 指针和调用方。

```toml
prelay-protocol = { path = "../crates/protocol" }
```

## 协议材料

[docs/protocol/](docs/protocol/) 是唯一的 Bruno 请求集合，按身份、供应商、接入点和统计资源组织，根目录保存四个 `/v1` 调用入口。每个请求的路径、方法、鉴权和示例正文应与本 crate DTO 及服务端路由一致。

`environments/template.bru` 只包含本地示例与占位符。复制后填写实际地址和凭据，个人环境及真实设备凭据、Endpoint Token、Provider API Key 均不得提交。

## 验证

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```
