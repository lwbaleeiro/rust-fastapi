
**Overview**
This project is a high-performance backend developed in Rust, showcasing production-ready architecture, authentication, concurrency, and database integration. It is designed to demonstrate backend engineering proficiency using the modern Rust ecosystem.

---

### **Technologies**

* **Rust (stable)**
* **Axum** – HTTP framework
* **Tokio** – async runtime
* **SQLx** – async typed PostgreSQL driver
* **PostgreSQL** – relational database
* **Argon2** – secure password hashing
* **JWT** – authentication and authorization
* **Tracing** – structured logging
* **Docker** – containerization

---

### **Architecture**

The system follows a layered architecture:

1. **API Layer**

   * Request routing
   * Validation
   * Extractors for auth and state

2. **Application Layer**

   * Services and use cases
   * Business logic orchestration

3. **Domain Layer**

   * Entities and enums
   * Pure domain logic

4. **Infrastructure Layer**

   * Database repositories
   * JWT engine
   * Hashing engine
   * Background workers (Tokio mpsc)

---

### **Main Features**

#### **Authentication**

* Registration
* Login
* JWT access token
* Refresh token flow
* Password hashing (Argon2)

#### **Task Manager**

* CRUD operations
* Per-user filtering
* Status transitions
* Full service + repository pattern

#### **Background Processing**

* Job queue using `tokio::mpsc`
* Worker loop running concurrently
* Example job: async email simulation

#### **Testing**

* Unit tests for service layer
* Integration tests for API routes
* Database-backed tests with SQLx
