模块路径: codex-rs/tui/src/bottom_pane/skills_toggle_view.rs
职责: 渲染技能开关列表并支持搜索/启停。
关键组件/接口: `SkillsToggleView`、`skills_toggle_hint_line`
关键依赖: `match_skill`、`render_rows_single_line`
改动说明:
- 时间: 2026-02-17
- 原因: 技能管理面板中文化。
- 影响: 标题、副标题、搜索占位、空态提示和底部操作提示均改为中文。
- 注意点: 调用方传入的技能名称/描述不做翻译。
后续优化:
- 增加技能描述翻译映射能力。
变更记录:
- 2026-02-17: 技能开关视图交互文案汉化。
