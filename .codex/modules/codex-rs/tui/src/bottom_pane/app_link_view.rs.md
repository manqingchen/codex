模块路径: codex-rs/tui/src/bottom_pane/app_link_view.rs
职责: 渲染应用安装/管理弹层，处理打开链接、启停应用与安装确认流程。
关键组件/接口: `AppLinkView`、`AppLinkViewParams`、`action_labels`、`hint_line`
关键依赖: `ListSelectionView`、`AppEvent::OpenUrlInBrowser`、`AppEvent::SetAppEnabled`
改动说明:
- 时间: 2026-02-17
- 原因: 持续推进 TUI 汉化，覆盖应用安装与管理流程文案。
- 影响: 弹层标题、操作按钮、步骤说明、底部提示全部改为中文。
- 注意点: 链接 URL 与事件行为未改，仅调整展示文本。
后续优化:
- 将应用相关文案抽离到统一 i18n 资源层。
变更记录:
- 2026-02-17: 完成 app 链接/安装流程汉化，并同步快照。
