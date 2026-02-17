模块路径: codex-rs/tui/src/bottom_pane/experimental_features_view.rs
职责: 展示实验功能开关列表并处理启停操作。
关键组件/接口: `ExperimentalFeaturesView`、`experimental_popup_hint_line`
关键依赖: `Feature`、`render_rows`、`AppEvent::SetFeatureFlag`
改动说明:
- 时间: 2026-02-17
- 原因: 补齐实验功能弹层中文提示。
- 影响: 标题、副标题与底部快捷提示改为中文。
- 注意点: 功能开关逻辑与事件流保持不变。
后续优化:
- 将实验功能描述统一纳入可翻译配置。
变更记录:
- 2026-02-17: 实验功能开关弹层完成汉化。
