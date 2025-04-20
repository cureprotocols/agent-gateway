![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Dockerized](https://img.shields.io/badge/docker-ready-blue)
![License](https://img.shields.io/badge/license-MIT-lightgrey)
[![Docker Pulls](https://img.shields.io/docker/pulls/cureprotocols/agent-gateway?style=flat-square)](https://hub.docker.com/r/cureprotocols/agent-gateway)

# 🧱 Agent Token Gateway

A high-performance, Rust-powered access control gateway for agent-based systems.  
Verifies JWT tokens, enforces scope-based permissions, and logs requests with full traceability.

> Built for stealth. Deployed for trust. Hardened for scale.

---

## 🚀 Features

- ✅ JWT authentication (`HS256`)
- ✅ Scope-based permission checks
- ✅ Clean error handling (401, 403)
- ✅ Structured JSON responses
- ✅ X-Request-ID support
- ✅ Tracing + logging middleware
- ✅ Dockerized deploy
- ✅ Full test coverage

---

## ⚙️ Tech Stack

| Tech | Purpose |
|------|---------|
| `Rust + Axum` | Blazing fast API performance |
| `jsonwebtoken` | Token validation |
| `tower-http` | Middleware (Request IDs, Tracing) |
| `dotenv` | Config + secret management |
| `tokio` | Async execution |
| `Docker` | Portable, cloud-ready builds |

---

## 📂 Project Structure

```plaintext
agent-token-gateway/
├── src/
│   ├── main.rs              # Entrypoint with trace layers
│   ├── config.rs            # AppState (env config)
│   ├── errors.rs            # Unified error system
│   ├── routes.rs            # Route definitions
│   ├── handlers/            # /check-access logic
│   ├── utils/               # JWT decoder + scope enforcer
│   ├── middleware/          # (Optional rate limiter)
│   └── lib.rs               # Library export for testing
├── tests/
│   └── core.rs              # Token + scope enforcement tests
├── .env                     # Runtime secrets (JWT_SECRET, etc.)
├── Cargo.toml               # Rust dependency manifest
├── Dockerfile               # Container build script
└── README.md                # You're here
```

---

## 📜 .env Example

```env
JWT_SECRET=super_secret_gateway_key_123
RUST_LOG=info
```

---

## 🔧 Getting Started

### 🔹 1. Build Locally

```bash
cargo build
```

### 🔹 2. Run the App

```bash
cargo run
```

Runs on:  
```
http://localhost:3000
```

---

### 🔹 3. Test the System

```bash
cargo test
```

You’ll see:

```bash
test test_scope_enforcement_allows_valid_action ... ok
test test_expired_token_claims ... ok
✅ All tests passed
```

---

### 🔹 4. Build Docker Image

```bash
docker build -t agent-token-gateway .
```

### 🔹 5. Run with Docker

```bash
docker pull cureprotocols/agent-gateway
docker run -p 3000:3000 --env-file .env cureprotocols/agent-gateway
The gateway will be live at: http://localhost:3000
```

---

## 🔐 Sample Request

```http
POST /check-access
Content-Type: application/json

{
  "token": "<your-jwt>",
  "action": "agent:read"
}
```

**Response:**

```json
{
  "allowed": true,
  "reason": "Access granted"
}
```

---

## 🧪 Security Notes

- HS256 key from `.env` is required to run
- Tokens must contain:
  - `sub`, `iat`, `exp`, `scopes`
- All scopes are matched exactly

---

## 📌 Future Enhancements

- ⛔ Rate limiting middleware
- 🔄 JWT key rotation support
- 🧩 Agent plugin system
- 🧠 LLM routing integration
- 🧱 Admin dashboard (Laravel, Nuxt)

---

## ✊ Built By

A stealth operator.  
Signal over noise.  
Rust over excuses.  
🔥 Powered by precision, trust, and execution.

---

