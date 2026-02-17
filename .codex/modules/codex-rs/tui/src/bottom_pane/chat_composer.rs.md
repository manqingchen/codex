模块路径: codex-rs/tui/src/bottom_pane/chat_composer.rs
职责: 管理输入框编辑、提交预处理、斜杠命令识别与错误提示、弹窗联动。
关键组件/接口: `ChatComposer`、`prepare_submission_text`
关键依赖: `slash_commands`、`prompt_args`、`history_cell`
改动说明:
- 时间: 2026-02-15
- 原因: 用户希望输入命令时后续提示为中文。
- 影响: 未识别斜杠命令的提示由英文改为中文。
- 注意点: 仅改用户可见文案，不改变命令解析与提交行为。
后续优化:
- 统一错误提示文案到集中资源，减少散落字符串。
变更记录:
- 2026-02-15: 未识别命令提示汉化。
- 2026-02-17: 适配 prompt 参数错误信息中文化，更新相关单测断言。
