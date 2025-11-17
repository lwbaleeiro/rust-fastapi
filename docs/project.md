# 🚀 Projeto para Portfólio em Rust

# **Rust FastAPI – High-Performance API with Axum, SQLx, JWT and Background Jobs**

---
System prompt:
```
Você é meu engenheiro de software sênior no projeto Rust FastAPI.
Seu papel é guiar, revisar, sugerir melhorias e apontar riscos — nunca escrever o código final para mim. Explique, questione, oriente e valide minhas decisões como um mentor experiente, mas deixe toda a implementação comigo.
```
---

# 🧠 Objetivo do Projeto

Construir uma API REST de alta performance em Rust com:

* **Axum** — web framework moderno
* **Tokio** — engine assíncrona
* **SQLx** — driver async para Postgres
* **JWT Auth** — login, registro, refresh token
* **Background Jobs** — usando canais async (Tokio mpsc)
* **Testes automatizados**
* **Logging estruturado** (tracing)
* **Migrations** (sqlx migrate)
* **Arquitetura modular e limpa**

---

# 🏗️ 1. Arquitetura Geral do Sistema

```
                        +-------------------------+
                        |     HTTP REST API       |
                        |        (Axum)           |
                        +------------+------------+
                                     |
                                     v
                        +-------------------------+
                        |      Application        |
                        |  Handlers / Use Cases   |
                        +------------+------------+
                                     |
                                     v
                        +-------------------------+
                        |         Domain          |
                        | Entities / Services     |
                        +------------+------------+
                                     |
                                     v
                        +-------------------------+
                        |       Infrastructure    |
                        | Postgres (SQLx)         |
                        | JWT                     |
                        | Background Jobs (mpsc)  |
                        +-------------------------+
```

* **Presentation (API)**: rotas HTTP, validações iniciais.
* **Application**: handlers, regras de negócio.
* **Domain**: entidades, serviços puros.
* **Infra**: banco, JWT, filas, logging.

---

# 📁 2. Estrutura de Pastas do Projeto (template)

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

# 🧩 3. Funcionalidades do Projeto

## **1. Auth (registro, login, refresh token)**

* Registro
* Login com JWT
* Refresh token
* Logout (revogação opcional)
* Hashing seguro (argon2)

---

## **2. CRUD de Tarefas (Task Manager)**

* Criar tarefa
* Listar tarefas
* Atualizar
* Concluir
* Deletar

Com validação por usuário.

---

## **3. Background Jobs (concorrência)**

Usando Tokio mpsc:

Exemplos de jobs:

* Enviar e-mail fictício após criação de usuário
* Processar relatórios
* Gerar logs pesados
* Simular fila (como Kafka, mas simples)

---

## **4. Integração com Postgres usando SQLx**

* Pool de conexões
* Queries tipadas
* Migrations automáticas

---

## **5. Logging Estruturado**

Usar `tracing` com spans e níveis.

---

## **6. Testes**

* Testes de unidade (serviços)
* Testes de integração (rotas reais + DB em memória ou docker)

---

# 🛣️ 4. Roadmap para Implementação (passo a passo)

### **Passo 1 — Setup inicial**

* Criar projeto Cargo
* Instalar Axum, SQLx, Tokio, Tracing
* Criar estrutura base de pastas

---

### **Passo 2 — Configuração (.env + Settings)**

* Variáveis (DB_URL, JWT_SECRET, PORT)
* Carregar com `dotenvy`

---

### **Passo 3 — Banco de Dados**

* Criar migrations:

  * tabela users
  * tabela tasks
* Configurar pool SQLx
* Criar repositories

---

### **Passo 4 — Domínio**

* Criar entidades User e Task
* Criar traits de repositório (interface)

---

### **Passo 5 — Infra**

* JWT + Refresh
* Hashing com argon2
* Repositórios concretos

---

### **Passo 6 — Services (regra de negócio)**

* UsersService
* TasksService
* JobQueueService

---

### **Passo 7 — API (rotas + controllers)**

Rotas principais:

```
POST   /auth/register
POST   /auth/login
POST   /auth/refresh

GET    /tasks
POST   /tasks
PUT    /tasks/:id
DELETE /tasks/:id
```

---

### **Passo 8 — Background Jobs**

Criar:

* mpsc::channel
* worker provando concorrência com Rust

---

### **Passo 9 — Testes**

* Testes de integração das rotas
* Testes de unidade dos services

---

### **Passo 10 — Otimizações / Prod**

* Dockerfile
* Configurar tracing no modo release
* Adicionar rate limiting
* Adicionar CORS

---
