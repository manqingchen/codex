模块路径: codex-rs/tui/src/bottom_pane/approval_overlay.rs
职责: 渲染命令/补丁/MCP 审批弹层并发送审批决策事件。
关键组件/接口: `ApprovalOverlay`、`exec_options`、`patch_options`、`elicitation_options`
关键依赖: `ListSelectionView`、`AppEvent::CodexOp`、`ReviewDecision`
改动说明:
- 时间: 2026-02-17
- 原因: 汉化审批流程提示与选项，统一中文交互体验。
- 影响: 审批标题、原因/服务端标签、确认提示、全部选项文案改为中文。
- 注意点: 中文宽字符会导致测试渲染插空，需要使用去空格匹配断言。
后续优化:
- 为审批选项增加按语言切换能力，避免硬编码。
变更记录:
- 2026-02-17: 完成审批弹层文案汉化并修正相关测试断言。
