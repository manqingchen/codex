模块路径: codex-rs/tui/src/bottom_pane/prompt_args.rs
职责: 解析自定义 prompt 参数并执行占位符展开。
关键组件/接口: `PromptArgsError`、`PromptExpansionError`、`expand_numeric_placeholders`
关键依赖: `shlex`、`regex_lite`、`CustomPrompt`
改动说明:
- 时间: 2026-02-17
- 原因: 将参数解析/缺参错误提示改为中文。
- 影响: 用户在自定义 prompt 参数错误时获得中文错误信息。
- 注意点: 协议字段与占位符语义保持不变，仅用户提示变化。
后续优化:
- 为错误提示提供可结构化错误码，减少字符串匹配依赖。
变更记录:
- 2026-02-17: prompt 参数错误提示汉化并调整相关测试断言。
