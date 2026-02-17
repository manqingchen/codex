模块路径: .github/workflows/bazel.yml
职责: 提供 Bazel 实验性跨平台构建与测试流程。
关键组件/接口:
- job: test
- step: bazel test //...
关键依赖:
- bazelbuild/setup-bazelisk@v3
- facebook/install-dotslash@v2
- secrets.BUILDBUDDY_API_KEY
改动说明:
- 时间: 2026-02-17
- 原因: fork 仓库在计费/Runner 能力上与上游差异大，导致实验性 Bazel 任务频繁非代码原因失败。
- 影响: Bazel 实验工作流仅在 openai/codex 执行，fork 中自动跳过。
- 注意点: 该改动针对 fork 可用性，不改变 upstream 的原执行路径。
后续优化:
- 可新增 fork 轻量 Bazel smoke test，保持基础覆盖同时避免高成本 runner。
变更记录:
- 2026-02-17: 为 Bazel test job 增加仓库级条件，避免 fork 环境不稳定失败。
