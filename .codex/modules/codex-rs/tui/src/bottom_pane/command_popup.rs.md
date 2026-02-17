模块路径: codex-rs/tui/src/bottom_pane/command_popup.rs
职责: 渲染斜杠命令与自定义 prompt 的选择弹窗，负责过滤、排序与展示描述。
关键组件/接口: `CommandPopup`、`rows_from_matches`、`CommandItem`
关键依赖: `selection_popup_common`、`slash_commands`、`CustomPrompt`
改动说明:
- 时间: 2026-02-15
- 原因: 用户希望输入命令时提示文案为中文。
- 影响: 自定义 prompt 在无描述时的兜底文案由英文改为中文；同步更新对应单测断言。
- 注意点: 用户自定义 frontmatter 描述不做翻译，仍按原文显示。
后续优化:
- 为 prompt 描述提供可选翻译钩子。
变更记录:
- 2026-02-15: 命令弹窗兜底描述汉化。
- 2026-02-17: 空态提示由英文 `no matches` 汉化为中文。
