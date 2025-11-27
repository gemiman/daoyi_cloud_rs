# Repository Guidelines

## 项目结构与目录
- `src/`: 主应用（配置、路由、Hoop 中间件、SeaORM 实体、入口 `main.rs`）。
- `crates/libs/daoyi-framework`: 共享框架库，预留通用组件。
- `crates/modules/daoyi-gateway`: 网关子服务示例。
- `migration/`: SeaORM Migrator CLI（生成/执行数据库迁移）。
- `views/` 与 `assets/`: Askama 模板与静态资源，前端改动需同步二者。
- `config-example.toml`: 配置示例，复制为 `config.toml` 后再启动。

## 构建、开发与测试命令
- 初始化配置：`cp config-example.toml config.toml`，必要时导出 `DATABASE_URL`、`APP_JWT__SECRET` 等环境变量。
- 启动服务：`cargo run`（读取 `config.toml`，默认监听 `127.0.0.1:8008`）。
- 数据库迁移：`cd migration && cargo run -- up`；回滚用 `cargo run -- down`；开发期重置用 `cargo run -- fresh`。
- 格式化与静态检查：`cargo fmt`、`cargo clippy -- -D warnings`，提交前必须通过。
- 测试：`cargo test`（涉及数据库的测试需先准备可用的 Postgres 并跑迁移）。
- 发布构建：`cargo build --release`，产物位于 `target/release/daoyi_cloud_rs`。

## 编码风格与命名约定
- Rust 2024，4 空格缩进，模块/文件 `snake_case`，类型 `CamelCase`，常量 `SCREAMING_SNAKE_CASE`。
- 避免 `unwrap`/`expect`，使用 `AppError` 或 `anyhow::Result` 统一错误返回；日志用 `tracing`，错误路径打上 `error!`/`warn!`。
- 路由处理保持返回别名 `JsonResult`/`EmptyResult`，新接口请补充 OpenAPI 注解。
- 公共 API 和复杂逻辑添加文档注释；保持 `clippy` 无警告。

## 测试准则
- 优先使用 `#[tokio::test]` 编写异步单测，命名 `test_*`；涉及数据库时先运行迁移并清理测试数据。
- 路由/中间件测试可用 `salvo::test::TestClient`，断言 HTTP 状态与 JSON 结构。
- 变更实体、模型或验证规则时补充回归测试；提交前至少跑 `cargo test`。

## 提交与 Pull Request
- Commit 信息遵循 Conventional Commits（例：`feat: add jwt refresh`、`fix: handle cors preflight`），一次提交聚焦单个修改点。
- 提交前确认通过 `fmt`、`clippy`、`test`；涉及迁移或配置的变更需在描述中注明执行步骤。
- PR 描述包含变更摘要、测试结果、关联 Issue；若修改 `views/` 或静态资源，附关键截图或说明交互。

## 配置与安全提示
- 不要提交真实密钥、证书或 `.env`，示例配置请基于 `config-example.toml`；证书放置 `certs/` 并在 `config.toml` 引用路径。
- JWT 密钥应为生产级随机值；日志避免输出用户密码、Token 等敏感字段。
- 数据库连接池、TLS、跨域等设置集中在 `src/config`，变更后同步更新示例配置与文档。***
