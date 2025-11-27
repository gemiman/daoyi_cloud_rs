# Repository Guidelines

## Project Structure & Modules
- Java 17 Spring Boot multi-module. Common starters/utilities live in `yudao-framework/`. Business modules are under `yudao-module-*` (each contains API + server). `yudao-server/` is the aggregate boot app; gateway at `yudao-gateway/`.
- Frontend scaffolds are placeholders in `yudao-ui/` (Vue3/Vben/Vue2/uni-app variants). SQL init scripts sit in `sql/`; local/compose scripts in `script/`.
- Main service config: `yudao-server/src/main/resources/application*.yaml`; override personal secrets in `application-local.yaml` only.

## Build, Test, and Development Commands
- `mvn -T 1C clean package -DskipTests` — full build, all modules, tests skipped.
- `mvn -pl yudao-server -am clean package -DskipTests` — package only the primary server with dependencies.
- `mvn -pl yudao-server -am spring-boot:run -DskipTests` — run locally; ensure Nacos/Redis/DB up (see `script/docker/docker-compose.yml`).
- Run focused tests, e.g., `mvn -pl yudao-module-system-server test`.

## Coding Style & Naming Conventions
- Follow Alibaba Java spec; 4-space indent; packages lower_snake; classes/methods camelCase.
- Prefer Lombok and MapStruct (enable annotation processing). Place shared constants in the relevant `yudao-framework` starter; reuse utilities from `yudao-common`.
- Keep DTO/VO/BO naming consistent across layers; avoid leaking domain-specific config into modules that already have starters.

## Testing Guidelines
- Tests live beside modules at `src/test/java`; use JUnit 5 + Spring Boot Test.
- Name integration or service tests `*Test`/`*IT`; cover core service/mapper logic for new features.
- For security/cache/MQ changes, add regression cases; prefer in-memory or mocked dependencies over real third-party services.

## Commit & Pull Request Guidelines
- Commit style: Conventional Commits (e.g., `feat(config): ...`, `fix(iot): ...`, `chore(project): ...`); highlight scope and impact.
- PRs should include: change summary, impacted modules, test results (command + outcome). Attach screenshots or interaction notes for UI changes.
- Reference issues in descriptions (e.g., `Fixes #123`); avoid mixing unrelated refactors with feature/fix PRs.

## Security & Configuration Tips
- Never commit secrets/keys. Store local credentials in `application-local.yaml` or environment variables.
- Production-grade limits/auth live in starters like `yudao-spring-boot-starter-protection`; document risk and rollback when adjusting them.
