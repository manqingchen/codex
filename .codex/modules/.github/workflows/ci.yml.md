模块路径: .github/workflows/ci.yml
职责: 负责仓库 JavaScript/文档相关基础 CI，包括依赖安装、npm 包暂存、README 校验与格式检查。
关键组件/接口:
- job: build-test
- step: Stage npm package
- step: Upload staged npm package artifact
关键依赖:
- pnpm/action-setup@v4
- actions/setup-node@v6
- facebook/install-dotslash@v2
- scripts/stage_npm_packages.py
改动说明:
- 时间: 2026-02-17
- 原因: fork 仓库缺少 release 分支流水线上下文，导致 npm 暂存步骤在 fork 中稳定失败。
- 影响: 仅在 openai/codex 执行 npm 暂存与产物上传；fork 仓库跳过该步骤，保留其余校验。
- 注意点: 若 fork 也需要发布流程，需要补齐 release 工作流上下文后再放开条件。
后续优化:
- 可增加显式日志说明跳过原因，减少排障成本。
变更记录:
- 2026-02-17: 为 Stage/Upload npm package 增加仓库条件，避免 fork 环境误失败。
