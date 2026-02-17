模块路径: codex-rs/tui/src/bottom_pane/list_selection_view.rs
职责: 提供通用可筛选列表选择视图（含标题、搜索、列表与底部提示）。
关键组件/接口: `ListSelectionView`、`SelectionViewParams`、`render_rows*`
关键依赖: `selection_popup_common`、`ScrollState`
改动说明:
- 时间: 2026-02-17
- 原因: 统一列表弹层的中文空态提示与头部折叠提示。
- 影响: `no matches` 与 `ctrl + a view all` 文案改为中文。
- 注意点: 搜索占位文本由调用侧传入，需调用方同步汉化。
后续优化:
- 将空态/折叠提示抽成常量，避免多处散落。
变更记录:
- 2026-02-17: 通用选择列表关键提示汉化。
