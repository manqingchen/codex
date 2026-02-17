模块路径: codex-rs/tui/src/bottom_pane/multi_select_picker.rs
职责: 提供可多选、可重排、可搜索的通用选择器。
关键组件/接口: `MultiSelectPicker`、`MultiSelectPickerBuilder`
关键依赖: `render_rows_single_line`、`ScrollState`
改动说明:
- 时间: 2026-02-17
- 原因: 统一多选器搜索占位、空态与默认操作提示为中文。
- 影响: 默认 `search` 占位、`no matches`、默认 footer hint 均已汉化。
- 注意点: 自定义 `instructions` 的调用方仍需自行提供中文。
后续优化:
- 引入全局文案源，避免组件内硬编码。
变更记录:
- 2026-02-17: 多选器基础交互提示汉化。
