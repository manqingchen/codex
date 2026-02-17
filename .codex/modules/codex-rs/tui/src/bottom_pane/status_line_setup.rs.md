模块路径: codex-rs/tui/src/bottom_pane/status_line_setup.rs
职责: 提供状态栏显示项配置视图与预览。
关键组件/接口: `StatusLineItem`、`StatusLineSetupView`
关键依赖: `MultiSelectPicker`、`AppEvent::StatusLineSetup`
改动说明:
- 时间: 2026-02-17
- 原因: 将状态栏配置页与条目描述统一为中文。
- 影响: 配置标题、说明、操作提示、条目描述与预览示例文本改为中文。
- 注意点: 条目 ID（kebab-case）保持不变，确保配置兼容。
后续优化:
- 分离“展示示例”和“真实值格式”策略，减少硬编码示例文本。
变更记录:
- 2026-02-17: 状态栏配置页文案汉化。
