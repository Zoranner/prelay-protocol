# Prelay 协议请求

本目录是 Prelay 唯一维护的 Bruno 请求集合。请求按 `身份`、`供应商`、`接入点` 和 `统计` 资源组织；根目录保存四个 AI 工具协议入口。DTO 和稳定错误码以本仓 Rust crate 为准，Bruno 请求是对应 HTTP 调用示例。

## 使用

在 Bruno 中打开本目录。复制 `environments/template.bru` 为个人环境后填写服务地址；请求中的尖括号占位值按实际资源填写。个人环境和真实凭据不得提交。

## 调用顺序

1. 在 `身份/注册.bru` 注册身份。
2. 在 `供应商/新增.bru` 新增供应商，并将响应中的 `id` 填入后续供应商或接入点请求的 `<provider-id>`。
3. 在 `接入点/新增.bru` 新增接入点，并将响应中的 `id` 和 `token` 分别填入 `<endpoint-id>` 与 `<endpoint-token>`。
4. 调用 `模型列表.bru`，将返回模型的 `id` 填入 `<endpoint-model-name>`。
5. 按所需协议调用 `创建响应.bru`、`创建对话补全.bru` 或 `创建消息.bru`。

## 鉴权

- `身份/注册.bru` 无需鉴权。
- 其余身份、供应商、接入点和统计请求使用设备凭据。
- `/v1` 请求使用接入点令牌，可通过 `Authorization: Bearer` 或 `X-Api-Key` 传递。

请求样例只使用占位符。Provider API Key 仅在供应商创建、更新、模型发现和协议测试请求中作为输入出现；接入点令牌不能替代 Provider API Key，Provider API Key 也不能调用 `/v1` 入口。

## 统计范围

统计请求的 `range` 可取 `today`、`yesterday`、`this_week`、`last_week`、`this_month`、`last_month`、`this_year`、`last_year` 或 `all`；省略时默认 `today`。统计按北京时间自然日边界聚合。
