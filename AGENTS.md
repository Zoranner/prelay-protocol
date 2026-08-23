# prelay-protocol

本仓只定义 `prelay-server` 与 `prelay-client` 共用的管理 API DTO、稳定错误码和协议请求材料；不承载 HTTP 路由、身份认证、数据库、加密存储、供应商适配或协议桥接。

## 契约规则

- 对外 DTO 是两个父仓的共享契约。字段名、可选性、默认值、枚举值和错误码的任何变化，都要先识别 server/client 两侧消费者与 submodule 更新需求。
- 保持 Rust 导入名 `prelay_protocol` 与 crate 包名 `prelay-protocol`。服务端和客户端都通过各自 `crates/protocol` 子模块的相对路径依赖本仓。
- Bruno 环境模板只能使用占位符，不得包含 device credential、Endpoint Token 或 Provider API Key。
- 不要为了父仓的局部实现方便把路由、存储或桌面细节引入此 crate。

## 验证

修改 Rust 代码后在仓库根目录执行：

```text
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

提交前执行 `git diff --check`。更新此仓后，只有两个父仓明确更新了 submodule 指针时，才能分别提交对应父仓。
