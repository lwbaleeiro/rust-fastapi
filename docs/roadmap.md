
---

# ✅ Roadmap

1. [Project Setup](#-1-project-setup)
2. [Database](#-2-database)
3. [Authentication](#-3-authentication)
4. [Task Manager](#-4-task-manager)
5. [Background Jobs](#-5-background-jobs)
6. [Observability](#-6-observability)
7. [Tests](#-7-tests)
8. [Docker](#-8-production-ready-docker)
---

### 🧱 **Lista: BACKLOG**

---

## 📦 **1. Project Setup**

**Card: Create Rust Project**
Checklist:

* Initialize cargo project
* Add Axum, SQLx, Tokio, Tracing
* Configure workspace layout

---

**Card: Create Folder Structure**
Checklist:

* Create modules: api, domain, application, infra
* Create migrations folder
* Setup initial files

---

**Card: Environment Configuration (.env)**
Checklist:

* Create .env.example
* Add DB_URL, JWT_SECRET, PORT
* Implement config loader

---

---

## 🗄️ **2. Database**

**Card: Create Users & Tasks Migrations**
Checklist:

* Users table
* Tasks table
* Apply migration

---

**Card: Setup SQLx Database Pool**
Checklist:

* Configure connection
* Implement DB health check

---

**Card: Build User Repository**
Checklist:

* Create/save/find operations
* Map SQL rows to Rust types

---

**Card: Build Task Repository**
Checklist:

* CRUD for tasks
* List tasks by user

---

---

## 🔐 **3. Authentication**

**Card: Implement Password Hashing (Argon2)**
Checklist:

* Hash passwords
* Verify passwords

---

**Card: Implement JWT Engine**
Checklist:

* Access token
* Refresh token
* Token validation

---

**Card: Auth Routes (Register, Login, Refresh)**
Checklist:

* Validate payload
* Return tokens
* Handle errors

---

---

## 📋 **4. Task Manager**

**Card: Create Task Entity**
Checklist:

* Title
* Description
* Status enum

---

**Card: Task Service (Business Logic)**
Checklist:

* Create
* Update
* Mark as done
* Delete

---

**Card: Task Routes (CRUD)**
Checklist:

* GET /tasks
* POST /tasks
* PUT /tasks/:id
* DELETE /tasks/:id

---

---

## ⚙️ **5. Background Jobs**

**Card: Implement Job Queue (mpsc)**
Checklist:

* Create channel
* Define job types

---

**Card: Worker System**
Checklist:

* Loop receiving jobs
* Logging and error handling

---

**Card: Example Job (Send Email Simulation)**
Checklist:

* Trigger job on user registration
* Write worker logic

---

---

## 📊 **6. Observability**

**Card: Configure Tracing**
Checklist:

* Set subscriber
* Add spans

---

**Card: Add Logs to Services and Controllers**
Checklist:

* Information logs
* Error logs
* Trace-level logs

---

---

## 🧪 **7. Tests**

**Card: Unit Tests (Services)**
Checklist:

* UsersService
* TasksService

---

**Card: Integration Tests (API + DB)**
Checklist:

* Test login
* Test task endpoints

---

---

## 🚀 **8. Production Ready (Docker)**

**Card: Dockerfile + docker-compose**
Checklist:

* Rust container
* Postgres container

---

**Card: CORS & Rate Limiting**
Checklist:

* Define allowed origins
* Use tower middleware

---