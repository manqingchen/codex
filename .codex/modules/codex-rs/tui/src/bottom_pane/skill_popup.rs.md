模块路径: codex-rs/tui/src/bottom_pane/skill_popup.rs
职责: 渲染技能提及选择弹层并处理筛选/选择状态。
关键组件/接口: `SkillPopup`、`rows_from_matches`、`skill_popup_hint_line`
关键依赖: `codex_utils_fuzzy_match`、`selection_popup_common`
改动说明:
- 时间: 2026-02-17
- 原因: 技能选择弹层空态与操作提示汉化。
- 影响: `no matches` 与 `insert/close` 提示改为中文。
- 注意点: 匹配与排序逻辑不变，仅文本展示变化。
后续优化:
- 与其他 popup 统一空态词条来源。
变更记录:
- 2026-02-17: 技能提及弹层交互提示汉化。
