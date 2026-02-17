模块路径: codex-rs/tui/src/cwd_prompt.rs
职责: 在恢复/分叉会话时提示用户选择工作目录。
关键组件/接口: `run_cwd_selection_prompt`、`CwdPromptScreen`
关键依赖: `Tui`、`selection_option_row`
改动说明:
- 时间: 2026-02-17
- 原因: 会话工作目录选择弹窗中文化。
- 影响: 标题、说明、选项、继续提示均改为中文。
- 注意点: 选择行为与默认回退策略（Esc 选 Session）未改。
后续优化:
- 将 `Session`/`Current` 标签本地化为可配置词条。
变更记录:
- 2026-02-17: cwd 选择弹窗文案汉化并同步快照。
