模块路径: .github/workflows/rust-ci.yml
职责: Rust 主 CI，负责格式、依赖分析、跨平台 lint/build、测试与统一结果汇总。
关键组件/接口:
- job: changed
- job: lint_build / tests（upstream）
- job: lint_build_fork / tests_fork（fork）
- job: results (CI results required)
关键依赖:
- dtolnay/rust-toolchain@1.93.0
- taiki-e/install-action（cargo-shear/sccache/nextest）
- actions/cache@v5
- facebook/install-dotslash@v2
改动说明:
- 时间: 2026-02-17
- 原因: fork 缺少 codex-runners runner group 且部分平台受计费限制，导致原矩阵大面积启动失败。
- 影响: upstream 继续使用原完整矩阵；fork 使用新增的 ubuntu 轻量 lint/test job，并由 results 按仓库类型判定通过条件。
- 注意点: results 仍为唯一 required 状态，避免分支保护规则频繁调整。
后续优化:
- 可将 fork/upstream 差异抽象为 reusable workflow，减少重复与维护成本。
变更记录:
- 2026-02-17: 增加 fork 专用轻量 CI job，并调整 results 聚合逻辑按仓库类型判定。
