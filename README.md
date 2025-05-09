# Seven Agent

一个基于 Rust 和 React 的智能代理系统。

## 项目结构

```
seven_agent/
├── src/                # Rust 后端代码
├── migrations/         # 数据库迁移文件
├── seven-agent-frontend/  # React 前端代码
└── .env               # 环境配置文件
```

## 技术栈

### 后端
- Rust
- Rocket (Web 框架)
- SQLx (数据库 ORM)
- PostgreSQL (数据库)
- JWT (身份认证)

### 前端
- React
- TypeScript
- Tailwind CSS
- Vite

## 环境要求

- Rust 1.70+
- Node.js 16+
- PostgreSQL 13+
- npm 或 yarn

## 快速开始

### 后端设置

1. 安装 Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 配置环境变量
```bash
cp .env.example .env
# 编辑 .env 文件，设置必要的环境变量
```

3. 运行数据库迁移
```bash
sqlx database create
sqlx migrate run
```

4. 启动后端服务
```bash
cargo run
```

### 前端设置

1. 进入前端目录
```bash
cd seven-agent-frontend
```

2. 安装依赖
```bash
npm install
```

3. 启动开发服务器
```bash
npm run dev
```

## API 文档

启动服务后，可以通过以下地址访问 API 文档：
- Swagger UI: http://localhost:8000/swagger-ui/

## 主要功能

- 用户认证（注册/登录）
- 密码重置
- JWT 令牌认证
- RESTful API
- Swagger API 文档

## 开发指南

### 添加新的 API 端点

1. 在 `src/routes` 目录下创建新的路由文件
2. 在 `src/main.rs` 中注册新的路由
3. 更新 Swagger 文档

### 数据库迁移

1. 创建新的迁移文件：
```bash
sqlx migrate add <migration_name>
```

2. 运行迁移：
```bash
sqlx migrate run
```

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

MIT License - 详见 LICENSE 文件 