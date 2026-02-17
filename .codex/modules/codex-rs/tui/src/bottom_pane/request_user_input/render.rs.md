模块路径: codex-rs/tui/src/bottom_pane/request_user_input/render.rs
职责: 负责 request_user_input 的布局计算与 UI 渲染。
关键组件/接口: `render_ui`、`render_unanswered_confirmation`、`unanswered_confirmation_layout`
关键依赖: `selection_popup_common`、`ratatui`
改动说明:
- 时间: 2026-02-17
- 原因: 将渲染层中的进度、空态、隐藏选项提示文案改为中文。
- 影响: `Question`/`No options`/`option x/y` 等可见文本均已汉化。
- 注意点: 与 `mod.rs` 的提示文案需保持一致，避免前后语义不一致。
后续优化:
- 提取渲染文本常量，减少布局文件中的文案耦合。
变更记录:
- 2026-02-17: request_user_input 渲染层中文化。
