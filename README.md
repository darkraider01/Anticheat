# AIGuard MVP - Day 1

This is the Day 1 implementation of the AIGuard MVP, a pragmatic monolith in Rust (backend) and static HTML/CSS (frontend).

## Project Structure

```
.
├── .env.example
├── Cargo.toml
├── README.md
├── build.rs
├── src
│   ├── auth
│   │   ├── api_key.rs
│   │   └── jwt.rs
│   ├── config
│   │   └── mod.rs
│   ├── handlers
│   │   ├── auth.rs
│   │   ├── dashboard_api.rs
│   │   ├── ingest.rs
│   │   ├── mod.rs
│   │   └── realtime.rs
│   ├── main.rs
│   ├── middleware
│   │   └── mod.rs
│   ├── models
│   │   └── mod.rs
│   ├── openapi
│   │   └── mod.rs
│   ├── router
│   │   └── mod.rs
│   ├── telemetry
│   │   └── mod.rs
│   └── lib.rs
└── web
    ├── auth.css
    ├── forgot.html
    ├── index.html
    ├── login.html
    └── styles.css
```

## Quick Start

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-repo/aiguard-mvp.git
    cd aiguard-mvp
    ```
2.  **Set up environment variables:**
    Copy `.env.example` to `.env` and fill in the necessary values.
    ```bash
    cp .env.example .env
    # Edit .env with your secrets
    ```
3.  **Run the backend:**
    ```bash
    cargo run --package anticheat
    ```
    The server will start on `0.0.0.0:3000`.

## Testing with `curl`

### Health Check

```bash
curl http://localhost:3000/healthz
# Expected output: {"status":"ok"}
```

### Version Info

```bash
curl http://localhost:3000/version
# Expected output: {"version":"0.1.0","git_sha":"<GIT_SHA>","build_time":"<BUILD_TIME>"}
```

### OpenAPI JSON

The generated OpenAPI specification can be found at:
[http://localhost:3000/api-docs/openapi.json](http://localhost:3000/api-docs/openapi.json)

You can also view the Swagger UI at:
[http://localhost:3000/api-docs/](http://localhost:3000/api-docs/)

### Authenticated Endpoints (Stubs)

**Login (Auth)**

```bash
curl -X POST -H "Content-Type: application/json" -d '{"email":"test@example.com","password":"password"}' http://localhost:3000/auth/login
# Expected output: {"token":"<stub>"}
```

**Ingest Batch (API Key)**

```bash
curl -X POST -H "X-API-Key: my-secret-api-key" -H "Content-Type: application/json" -d '{"events":[{"event_type":"login","payload":{"user_id":"123"}}]}' http://localhost:3000/ingest/batch
# Expected output: (HTTP 202 Accepted)
```

**Dashboard API (JWT Token)**

First, get a dummy token from the login endpoint. Then use it for dashboard API calls.

```bash
# Replace <YOUR_DUMMY_JWT_TOKEN> with the token from the login response
curl -H "Authorization: Bearer <YOUR_DUMMY_JWT_TOKEN>" http://localhost:3000/v1/detections
# Expected output: [{"id":"det-0","org_id":"org456","severity":"high","created_at":"2024-01-01T12:00:00Z"}]
```

## Frontend

Open `web/index.html` and `web/login.html` directly in your browser to view the static frontend.

## Day 2 Checklist

*   [ ] Implement actual database calls (e.g., DB ping in `/healthz`).
*   [ ] Implement JWT validation with HS256/RS256.
*   [ ] Implement API-key lookup and validation.
*   [ ] Wire pagination to actual database queries for dashboard APIs.
*   [ ] Add more comprehensive tests for all endpoints and middleware.
*   [ ] Implement WebSocket actual logic in `/ws/dashboard`.