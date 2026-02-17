模块路径: codex-rs/tui/src/bottom_pane/selection_popup_common.rs
职责: 提供选择弹层共享渲染能力（列宽、换行、空态、禁用态）。
关键组件/接口: `GenericDisplayRow`、`render_rows*`、`build_full_line`
关键依赖: `unicode_width`、`ratatui`
改动说明:
- 时间: 2026-02-17
- 原因: 统一禁用态描述文案为中文。
- 影响: 行描述中的 `disabled` 标记改为 `已禁用`。
- 注意点: 该模块被多个弹层复用，文案变化会扩散到多处快照。
后续优化:
- 将状态前缀规范化，支持不同上下文自定义状态词。
变更记录:
- 2026-02-17: 共享禁用态文案汉化。
