模块路径: codex-rs/tui/src/bottom_pane/request_user_input/mod.rs
职责: 管理 request_user_input 交互状态机、问题导航、答案提交与底部提示。
关键组件/接口: `RequestUserInputOverlay`、`footer_tips`、`unanswered_confirmation_rows`
关键依赖: `ChatComposer`、`RequestUserInputEvent`、`AppEvent`
改动说明:
- 时间: 2026-02-17
- 原因: 补齐问答弹层交互文案汉化。
- 影响: 占位文本、底部 tips、未答确认标题/按钮/描述改为中文。
- 注意点: 中文宽字符会影响渲染宽度和快照，需同步更新快照与断言。
后续优化:
- 将问答提示拆为可配置词条，支持后续多语言扩展。
变更记录:
- 2026-02-17: request_user_input 状态机用户提示全面汉化。
