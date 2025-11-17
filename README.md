# **Rust FastAPI — High-Performance API with Axum, SQLx, JWT & Background Jobs**

> [Read this in English](README_EN.md)

Um projeto completo em Rust demonstrando:

* Web API de alta performance
* Autenticação JWT
* Estrutura profissional
* Concorrência com background workers
* Migrations com SQLx
* Arquitetura limpa e modular

---

## ✨ Funcionalidades

* Registro e login com JWT
* Refresh token
* CRUD de tarefas (Task Manager)
* Background jobs com Tokio mpsc
* Logging estruturado
* Serviços separados por camadas
* Repositórios testáveis
* Testes automatizados
* Conexão com Postgres usando SQLx

---

## 🏗️ Arquitetura

```
API (Axum)
Application (Services)
Domain (Entities)
Infrastructure (DB, JWT, Jobs)
```

---

## 🔧 Tecnologias

* Rust
* Axum
* SQLx
* Tokio
* Tracing
* Argon2
* dotenvy
* JWT

---

## 🚀 Como rodar

```bash
cargo run
```

---

## 🧪 Testes

```bash
cargo test
```

---

## 📂 Estrutura de Pastas

```
rust-fastapi/
│
├── Cargo.toml
├── README.md
├── .env.example
├── .gitignore
│
├── migrations/                    # SQLx migrations
│   ├── 20250101_create_users.sql
│   └── ...
│
├── src/
│   ├── main.rs                    # bootstrap da aplicação
│   ├── lib.rs                     # inicialização comum
│
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs            # leitura do .env
│
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs              # definição de rotas
│   │   ├── extractors.rs          # auth, headers, context
│   │   └── responses.rs           # resposta padrão
│
│   ├── controllers/
│   │   ├── mod.rs
│   │   └── users_controller.rs
│   │   └── tasks_controller.rs
│
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── user.rs                # entidade User
│   │   └── task.rs                # entidade Task
│
│   ├── application/
│   │   ├── mod.rs
│   │   ├── users_service.rs       # regra de negócio
│   │   └── tasks_service.rs
│
│   ├── infra/
│   │   ├── mod.rs
│   │   ├── db.rs                  # pool SQLx
│   │   ├── jwt.rs                 # login, tokens
│   │   ├── hashing.rs             # bcrypt/argon2
│   │   ├── repositories/
│   │   │   ├── mod.rs
│   │   │   ├── users_repository.rs
│   │   │   └── tasks_repository.rs
│   │   └── jobs/
│   │       ├── mod.rs
│   │       ├── job_queue.rs       # mpsc channel
│   │       └── workers.rs         # background workers
│
│   ├── utils/
│   │   ├── mod.rs
│   │   └── errors.rs              # erros customizados
│
│
└── tests/
    ├── integration_users.rs
    ├── integration_tasks.rs
    └── healthcheck.rs
```

---