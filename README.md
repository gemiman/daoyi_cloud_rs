# Daoyi Cloud RS - 项目索引报告

## 📋 项目概述

**项目名称**: daoyi_cloud_rs  
**版本**: 0.1.1  
**Rust 版本**: 1.89  
**Edition**: 2024  
**项目类型**: 异步微服务 Web 应用

这是一个基于 Rust 构建的现代化微服务云平台，采用 Cargo 工作区组织多模块架构，旨在提供高性能、类型安全的 Web 服务。项目使用 Salvo 框架作为 Web 引擎，SeaORM 进行数据持久化，配合 Nacos 服务发现、Redis 缓存等基础设施，构建完整的企业级应用解决方案。

---

## 🏗️ 技术架构

### 核心技术栈

| 组件类别 | 技术选型 | 版本 | 用途说明 |
|---------|---------|------|----------|
| **Web 框架** | Salvo | 0.85.0 | 异步 Web 服务器，支持 OpenAPI、CORS、JWT 认证等 |
| **ORM** | SeaORM | 2.0.0-rc.19 | 数据库 ORM，支持 PostgreSQL |
| **异步运行时** | Tokio | 1.48.0 | 异步任务执行引擎（full features） |
| **服务发现** | nacos-sdk | 0.5.3 | 微服务注册与配置管理 |
| **缓存** | Redis | 1.0.0-rc.4 | 分布式缓存与会话存储 |
| **认证** | jsonwebtoken | 10.2.0 | JWT 令牌生成与验证 |
| **密码加密** | Argon2 | 0.5.3 | 安全密码哈希 |
| **配置管理** | Figment | 0.10.19 | 多源配置加载（TOML + 环境变量） |
| **日志追踪** | Tracing | 0.1.41 | 结构化日志与分布式追踪 |
| **模板引擎** | Askama | 0.14.0 | HTML 模板渲染 |
| **序列化** | Serde | 1.0.228 | JSON 序列化/反序列化 |
| **校验** | Validator | 0.20.0 | 数据校验与约束检查 |
| **ID 生成** | ULID | 1.2.1 | 分布式唯一 ID 生成 |
| **TLS** | RustLS | - | HTTPS 安全传输层 |

### 架构特性

- ✅ **Cargo 工作区**: 多 crate 模块化组织
- ✅ **异步优先**: 基于 Tokio 的全异步架构
- ✅ **类型安全**: 强类型 ORM + 请求/响应校验
- ✅ **OpenAPI**: 自动生成 API 文档（Scalar UI）
- ✅ **热重载配置**: Figment 多源配置支持
- ✅ **优雅关闭**: 信号处理与 60s 超时
- ✅ **TLS 支持**: 可选 HTTPS 加密传输

---

## 📂 项目结构

### 工作区成员

```toml
[workspace]
members = [
    ".",                          # 主应用
    "crates/libs/daoyi-framework", # 框架库（共享组件）
    "crates/modules/*",            # 业务模块（如 daoyi-gateway）
    "migration",                   # 数据库迁移工具
]
```

### 目录结构

```
.
├── src/                          # 主应用源码
│   ├── main.rs                   # 应用入口（服务启动、TLS、信号处理）
│   ├── error.rs                  # 统一错误处理（AppError 枚举）
│   ├── config/                   # 配置模块
│   │   ├── mod.rs                # 配置加载器（Figment）
│   │   ├── db_config.rs          # 数据库配置
│   │   └── log_config.rs         # 日志配置
│   ├── db/                       # 数据库连接池
│   │   └── mod.rs                # SeaORM 连接池管理
│   ├── entities/                 # SeaORM 实体
│   │   ├── mod.rs
│   │   ├── prelude.rs
│   │   └── users.rs              # 用户实体
│   ├── models/                   # 业务模型
│   │   └── mod.rs                # SafeUser 等传输对象
│   ├── routers/                  # 路由与控制器
│   │   ├── mod.rs                # 根路由（OpenAPI、静态资源）
│   │   ├── auth.rs               # 认证接口（登录、JWT 签发）
│   │   ├── user.rs               # 用户 CRUD（分页、过滤）
│   │   └── demo.rs               # 示例接口
│   ├── hoops/                    # 中间件（Salvo Hoop）
│   │   ├── mod.rs
│   │   ├── jwt.rs                # JWT 认证中间件
│   │   ├── cors.rs               # CORS 跨域处理
│   │   └── custom_middleware_example.rs
│   └── utils/                    # 工具函数
│       └── mod.rs                # 密码哈希等工具
│
├── crates/                       # 子 crate 目录
│   ├── libs/daoyi-framework/     # 框架库（待扩展）
│   │   └── src/lib.rs
│   └── modules/daoyi-gateway/    # 网关模块（独立服务）
│       └── src/main.rs
│
├── migration/                    # SeaORM 迁移工具
│   ├── src/
│   │   ├── main.rs               # 迁移 CLI
│   │   ├── lib.rs                # Migrator 实现
│   │   └── m20220101_000001_create_table.rs  # 迁移文件示例
│   ├── README.md                 # 迁移命令说明
│   └── AGENTS.md                 # 项目指南（含 Java 重构计划）
│
├── views/                        # Askama HTML 模板
│   ├── login.html                # 登录页面
│   ├── user_list_page.html       # 用户列表（完整页面）
│   ├── user_list_frag.html       # 用户列表（HTMX 片段）
│   ├── hello.html                # 演示模板
│   └── error_404.html            # 404 错误页
│
├── assets/                       # 前端静态资源
│   └── js/sweetalert2.js         # 前端库
│
├── Cargo.toml                    # 工作区与主应用依赖
├── askama.toml                   # 模板引擎配置
├── config-example.toml           # 配置文件示例
└── 重构计划.md                    # Java → Rust 迁移计划
```

---

## 🔑 核心模块说明

### 1. 应用入口 (`src/main.rs`)

**职责**:
- 加载配置（支持环境变量覆盖）
- 初始化数据库连接池
- 配置日志系统（Tracing + 可选 JSON 输出）
- 构建路由树（业务路由 + OpenAPI 文档）
- 启动 HTTP/HTTPS 服务器
- 优雅关闭（监听 SIGTERM/CTRL+C，60s 超时）

**关键特性**:
```rust
// 支持 TLS 证书
if let Some(tls) = &config.tls {
    let config = RustlsConfig::new(...).cert(...).key(...);
}

// 信号处理
tokio::select! {
    _ = ctrl_c => info!("ctrl_c signal received"),
    _ = terminate => info!("terminate signal received"),
}
```

### 2. 错误处理 (`src/error.rs`)

定义 `AppError` 枚举，统一处理：
- HTTP 状态错误（Salvo 内部错误）
- 数据库错误（SeaORM）
- 校验错误（Validator）
- 业务错误（公开/内部）

实现 `Writer` trait 自动序列化为 JSON 响应，并注册到 OpenAPI schema。

### 3. 配置管理 (`src/config/`)

**加载优先级**: `config.toml` < 环境变量 `APP_*` < `DATABASE_URL`

```toml
listen_addr = "127.0.0.1:8008"

[db]
url = "postgres://user:pass@localhost/db"
max_connections = 100

[jwt]
secret = "your-secret-key"
expiry = 86400  # 秒

[tls]  # 可选
cert = "/path/to/cert.pem"
key = "/path/to/key.pem"
```

### 4. 数据库层 (`src/db/` + `src/entities/`)

**连接池**:
- 使用 SeaORM 的 `ConnectOptions`
- 支持连接超时、空闲超时、最大/最小连接数配置
- 全局单例 `SEAORM_POOL`

**实体示例** (`users.rs`):
```rust
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,         // ULID
    pub username: String,   // 唯一索引
    pub password: String,   // Argon2 哈希
}
```

### 5. 路由系统 (`src/routers/`)

**根路由** (`mod.rs`):
```rust
Router::new()
    .get(demo::hello)                              // GET /
    .push(Router::with_path("login").get(...))     // GET /login
    .push(Router::with_path("api")
        .push(Router::with_path("login").post(...))     // POST /api/login
        .push(Router::with_path("users")
            .hoop(auth_hoop)                            // JWT 鉴权中间件
            .get(list_users)                            // GET /api/users
            .post(create_user)                          // POST /api/users
            .push(Router::with_path("{user_id}")
                .put(update_user)                       // PUT /api/users/:id
                .delete(delete_user)))                  // DELETE /api/users/:id
    )
```

**用户接口** (`user.rs`):
- `GET /api/users`: 分页查询（支持 `username` 模糊搜索）
- `POST /api/users`: 创建用户（校验用户名/密码长度）
- `PUT /api/users/{id}`: 更新用户
- `DELETE /api/users/{id}`: 删除用户

所有接口返回类型:
```rust
type JsonResult<T> = Result<Json<T>, AppError>;
type EmptyResult = Result<Json<Empty>, AppError>;
```

**认证接口** (`auth.rs`):
- `GET /login`: 登录页面（Askama 渲染）
- `POST /api/login`: 用户名密码登录，返回 JWT Token（存入 Cookie）

### 6. 中间件 (`src/hoops/`)

**JWT 认证** (`jwt.rs`):
```rust
pub fn auth_hoop(config: &JwtConfig) -> impl Handler {
    // 从 Cookie 或 Authorization Header 提取 Token
    // 验证签名与过期时间
    // 失败返回 401 Unauthorized
}
```

**CORS** (`cors.rs`):
```rust
pub fn cors_hoop() -> Cors {
    Cors::new()
        .allow_origin("*")
        .allow_methods(["GET", "POST", "PUT", "DELETE"])
        .allow_headers(["Content-Type", "Authorization"])
}
```

**404 处理** (`mod.rs`):
```rust
#[handler]
pub async fn error_404(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    if res.status_code == StatusCode::NOT_FOUND {
        res.render(Error404Template { ... });
    }
}
```

### 7. 工具库 (`src/utils/`)

- **密码哈希**: `hash_password(plain) -> Result<String>` (Argon2)
- **密码验证**: `verify_password(plain, hash) -> Result<bool>`
- **ID 生成**: `Ulid::new().to_string()`

### 8. 迁移系统 (`migration/`)

基于 SeaORM Migrator，支持命令:

```bash
# 生成新迁移
cargo run -- migrate generate MIGRATION_NAME

# 应用所有待迁移
cargo run -- up

# 回滚最后一次迁移
cargo run -- down

# 重置数据库（删表 + 重新迁移）
cargo run -- fresh

# 检查迁移状态
cargo run -- status
```

---

## 🚀 快速开始

### 环境要求

- Rust 1.89+
- PostgreSQL 数据库
- Redis（可选，用于缓存）
- Nacos（可选，用于服务发现）

### 配置步骤

1. **复制配置文件**
   ```bash
   cp dy_config-example.toml dy_config.toml
   ```

2. **修改 `config.toml`**
   ```toml
   [db]
   url = "postgres://username:password@localhost:5432/daoyi_cloud"
   
   [jwt]
   secret = "your-super-secret-key-change-in-production"
   expiry = 86400
   ```

3. **设置环境变量（可选）**
   ```bash
   export DATABASE_URL="postgres://..."
   export APP_JWT__SECRET="..."
   ```

4. **运行数据库迁移**
   ```bash
   cd migration
   cargo run -- up
   ```

5. **启动服务**
   ```bash
   cargo run
   ```

6. **访问服务**
   - 主页: http://127.0.0.1:8008/
   - API 文档: http://127.0.0.1:8008/scalar
   - 登录页面: http://127.0.0.1:8008/login

---

## 📡 API 接口

### 认证接口

| 方法 | 路径 | 说明 | 鉴权 |
|------|------|------|------|
| GET | `/login` | 登录页面 | ❌ |
| POST | `/api/login` | 用户登录 | ❌ |

**登录请求示例**:
```json
POST /api/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password123"
}
```

**响应示例**:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "01H2X3Y4Z5A6B7C8D9E0F1G2H3",
    "username": "admin"
  }
}
```

### 用户管理接口

| 方法 | 路径 | 说明 | 鉴权 |
|------|------|------|------|
| GET | `/api/users` | 用户列表（分页） | ✅ JWT |
| POST | `/api/users` | 创建用户 | ✅ JWT |
| PUT | `/api/users/{id}` | 更新用户 | ✅ JWT |
| DELETE | `/api/users/{id}` | 删除用户 | ✅ JWT |

**查询参数** (GET `/api/users`):
- `username` (可选): 用户名模糊搜索
- `current_page` (默认 1): 当前页码
- `page_size` (默认 10): 每页数量

**创建用户请求**:
```json
POST /api/users
Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "username": "newuser",
  "password": "securepass123"
}
```

**校验规则**:
- `username`: 最小长度 5
- `password`: 最小长度 6

---

## 🔐 安全特性

- ✅ **Argon2 密码哈希**: 抵御彩虹表攻击
- ✅ **JWT 令牌认证**: 无状态会话管理
- ✅ **CORS 配置**: 跨域资源共享控制
- ✅ **HTTPS 支持**: TLS 加密传输（可选）
- ✅ **输入校验**: Validator crate 自动校验
- ✅ **SQL 注入防护**: SeaORM 参数化查询

---

## 🧪 测试

项目包含集成测试示例 (`src/main.rs`):

```rust
#[tokio::test]
async fn test_hello_world() {
    config::init();
    let service = Service::new(crate::routers::root());
    let content = TestClient::get(...).send(&service).await;
    assert_eq!(content, "Hello World from salvo");
}
```

运行测试:
```bash
cargo test
```

---

## 📊 性能优化

- **异步 I/O**: Tokio 全异步处理请求
- **连接池**: 复用数据库连接，减少握手开销
- **静态资源嵌入**: `rust-embed` 编译时嵌入资源
- **零拷贝**: Salvo 高效请求/响应处理
- **压缩中间件**: 可选 GZIP/Brotli 响应压缩

---

## 🛠️ 开发工具

### 推荐 IDE

- RustRover
- VSCode + rust-analyzer

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy -- -D warnings
```

### 监听文件变化（热重载）

```bash
cargo install cargo-watch
cargo watch -x run
```

---

## 📦 构建与部署

### 生产构建

```bash
cargo build --release
```

二进制文件位于: `target/release/daoyi_cloud_rs`

### Docker 部署

```dockerfile
# Dockerfile 示例
FROM rust:1.89 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates
COPY --from=builder /app/target/release/daoyi_cloud_rs /usr/local/bin/
COPY --from=builder /app/views /app/views
ENV APP_CONFIG=/app/config.toml
EXPOSE 8008
CMD ["daoyi_cloud_rs"]
```

构建与运行:
```bash
docker build -t daoyi-cloud-rs .
docker run -p 8008:8008 --env-file .env daoyi-cloud-rs
```

---

## 🗺️ 未来规划

根据 `重构计划.md`，项目目标是从 Java 版本迁移到 Rust，逐步实现：

### 短期目标
- [ ] 完善 `daoyi-framework` 公共库（错误码、分页、时间工具）
- [ ] 实现 Nacos 配置中心集成
- [ ] Redis 缓存封装（分布式锁、PubSub）
- [ ] 完善 RBAC 权限系统（角色、菜单、数据权限）
- [ ] 操作日志中间件（审计日志落库）

### 长期规划
- [ ] 多租户支持（tenant_id 隔离）
- [ ] 短信/邮件通知模块
- [ ] OSS 文件上传（本地/云存储）
- [ ] 代码生成器（SeaORM Entity + Salvo Router）
- [ ] 业务模块迁移（system → infra → member → pay → mall → ...）

详见: [`重构计划.md`](重构计划.md)

---

## 📝 数据库迁移命令

- Generate a new migration file
    ```sh
    cargo run -- migrate generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```

---

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

### 开发流程

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 提交 Pull Request

### 编码规范

- 遵循 Rust 官方代码风格（使用 `cargo fmt`）
- 所有公开 API 必须包含文档注释
- 新功能需附带单元测试
- 提交信息遵循 Conventional Commits

---

## 📄 许可证

本项目采用 MIT 许可证

---

## 🔗 相关资源

- [Salvo 官方文档](https://salvo.rs/)
- [SeaORM 文档](https://www.sea-ql.org/SeaORM/)
- [Tokio 指南](https://tokio.rs/)
- [Rust 异步编程](https://rust-lang.github.io/async-book/)

---

## 📧 联系方式

如有问题或建议，请通过 Issue 反馈或联系维护者。

---

**最后更新**: 2025-11-27  
**索引生成**: 自动生成（基于代码库分析）
