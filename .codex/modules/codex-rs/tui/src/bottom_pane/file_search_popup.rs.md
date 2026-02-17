模块路径: codex-rs/tui/src/bottom_pane/file_search_popup.rs
职责: 渲染文件搜索下拉结果与加载态。
关键组件/接口: `FileSearchPopup`、`set_query`、`set_matches`
关键依赖: `codex_file_search::FileMatch`、`render_rows`
改动说明:
- 时间: 2026-02-17
- 原因: 将搜索弹层空态与加载态提示改为中文。
- 影响: `loading...` / `no matches` 改为中文文案。
- 注意点: 仅文案变更，不影响搜索匹配逻辑。
后续优化:
- 增加可配置 loading/empty 文案，统一多弹层体验。
变更记录:
- 2026-02-17: 文件搜索弹层状态文案汉化。
