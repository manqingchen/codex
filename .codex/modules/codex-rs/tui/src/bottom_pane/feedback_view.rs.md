模块路径: codex-rs/tui/src/bottom_pane/feedback_view.rs
职责: 处理反馈收集、日志上传确认与反馈结果提示。
关键组件/接口: `FeedbackNoteView`、`feedback_selection_params`、`feedback_upload_consent_params`
关键依赖: `codex_feedback`、`AppEvent::OpenFeedbackNote`、`history_cell`
改动说明:
- 时间: 2026-02-17
- 原因: 补齐反馈流程汉化，包含选择、输入、上传结果文案。
- 影响: 反馈分类、占位文案、上传确认、成功/失败提示均改为中文。
- 注意点: 分类编码（`bad_result` 等）保持英文以兼容后端协议。
后续优化:
- 将员工/外部用户分支文案抽取为模板，减少重复。
变更记录:
- 2026-02-17: 反馈流程中文化并同步相关快照。
