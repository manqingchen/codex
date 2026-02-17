模块路径: codex-rs/tui/src/slash_command.rs
职责: 定义内置斜杠命令枚举、命令描述、可用性与可见性规则。
关键组件/接口: `SlashCommand`、`SlashCommand::description`、`built_in_slash_commands`
关键依赖: `strum`、`strum_macros`
改动说明:
- 时间: 2026-02-15
- 原因: 用户希望命令输入时的提示改为中文。
- 影响: 所有内置斜杠命令描述文案改为中文，命令字本身保持不变。
- 注意点: 命令语义与可用性逻辑未变，仅改展示文案。
后续优化:
- 增加 i18n 层按语言动态返回命令描述。
变更记录:
- 2026-02-15: 斜杠命令描述汉化。
