# AI Agent Instructions for Medicine Manager (medman)

## Overview

**Project**: Medicine Manager - A web application for tracking medication effects and side effects using AI-generated review plans.

**Stack**:
- **Backend**: Rust (Axum + Tokio) with MongoDB
- **Frontend**: Vue.js 3 + Pinia + Tailwind CSS + Vite
- **AI**: Mistral API for plan generation
- **Auth**: JWT with bcrypt
- **Docker**: Full containerized setup (backend, frontend, MongoDB, Mongo Express)

**Purpose**: Users login, request AI-generated medication review plans (e.g., "I'm taking Ritalin, track digestive side effects"), then use daily forms to log symptoms and progress.

---

## Agent Guidelines

### Be Short
- **Max 150 words per response**. Code speaks for itself.
- No greetings, petits, hedging, or puffery.
- Start with code/structure, then minimal context if needed.

### Code Changes
- **Surgical changes only**: Modify only what was requested.
- **No speculative features, abstractions, or extra error handling**.
- Match existing style: indentation (4 spaces), naming, comment density.
- Read file before editing. Use exact string matches.
- Verify: run tests, read back file, confirm build.

### Communication Style
- **Structure first**: Lead with diagrams, code blocks, tables, or trees.
- **No emoji, no fluff words** (robust, seamless, elegant, powerful, flexible).
- **No tutorials**: Don't explain concepts user clearly knows.
- **Verify, don't assert**: If unsure, use tools to check.

---

## Project Structure

```
medman/
├── src/                          # Rust Backend
│   ├── main.rs                   # Router, CORS, static files
│   ├── auth.rs                   # JWT token creation/validation
│   ├── database.rs               # MongoDB connection
│   ├── error.rs                  # AppError enum
│   ├── models.rs                 # Data structures (User, Plan, Review, etc.)
│   └── routes/                   # API endpoints
│       ├── mod.rs                # Route exports
│       ├── auth.rs               # /api/auth/signup, /api/auth/login
│       ├── plans.rs              # /api/plans (CRUD)
│       ├── checkins.rs           # /api/checkins (daily logging)
│       ├── insights.rs            # /api/insights (analytics)
│       ├── medication.rs         # Legacy /api/medication/* routes
│       ├── health.rs             # /api/health
│       └── not_found.rs          # 404 handler
├── frontend/                     # Vue.js 3 Frontend
│   ├── src/
│   │   ├── views/                # Pages
│   │   ├── components/           # Reusable UI
│   │   ├── stores/               # Pinia state
│   │   ├── services/             # API client (Axios)
│   │   ├── router/               # Vue Router
│   │   ├── locales/              # i18n translations
│   │   ├── App.vue
│   │   └── main.js
│   ├── Dockerfile
│   └── vite.config.js
├── Cargo.toml                    # Rust dependencies
├── Dockerfile                    # Backend Dockerfile
├── docker-compose.yml            # All services
├── .env.example                  # Environment template
├── README.md                     # Setup instructions
└── AGENTS.md                     # This file
```

---

## Key Files Reference

| File | Purpose | Key Types/Functions |
|------|---------|---------------------|
| `src/models.rs` | Data models | `User`, `Plan`, `Review`, `Checkin`, `Insight` |
| `src/auth.rs` | Auth helpers | `create_token()`, `verify_token()`, `hash_password()` |
| `src/database.rs` | DB connection | `init_db()`, `get_collection<T>()` |
| `src/error.rs` | Error handling | `AppError` enum with Axum conversion |
| `src/routes/plans.rs` | Plan CRUD | `create_plan()`, `get_plans()`, `get_plan_series()` |
| `src/routes/checkins.rs` | Daily logging | `log_checkin()` |
| `src/routes/medication.rs` | Legacy routes | `generate_plan()`, `create_review()` |

---

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/health` | Health check |
| POST | `/api/auth/signup` | Register user |
| POST | `/api/auth/login` | Login, get JWT |
| GET | `/api/plans` | Get user's plans |
| POST | `/api/plans` | Create plan |
| GET | `/api/plans/:id/series` | Get plan checkin series |
| POST | `/api/checkins` | Log daily checkin |
| GET | `/api/insights` | Get analytics |
| GET | `/api/medication/plans` | Legacy: get plans |
| POST | `/api/medication/plans` | Legacy: generate plan |
| GET | `/api/medication/reviews` | Legacy: get reviews |
| POST | `/api/medication/reviews` | Legacy: create review |

---

## Common Patterns

### Backend (Rust/Axum)

**State Access**:
```rust
// In main.rs
let shared_state = AppState { db: db.clone() };
let app = Router::new().with_state(shared_state);

// In route handlers
pub async fn handler(State(state): State<AppState>) { ... }
```

**MongoDB Collection**:
```rust
use crate::database::get_collection;
let collection = get_collection::<User>(&state.db, "users").await;
```

**JWT Auth**:
```rust
// Verify token from Authorization header
let token = headers.get("Authorization").and_then(|h| h.to_str().ok())?;
let claims = verify_token(token.trim_start_matches("Bearer "))?;
```

### Frontend (Vue 3)

**API Calls** (Axios in `frontend/src/services/`):
```javascript
import axios from 'axios';
const api = axios.create({ baseURL: '/api' });
// Add JWT interceptor
export default api;
```

**Pinia Store** (`frontend/src/stores/`):
```javascript
import { defineStore } from 'pinia';
export const useAuthStore = defineStore('auth', { ... });
```

---

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `MONGODB_URI` | Yes | `mongodb://root:example@mongodb:27017/medman?authSource=admin` | MongoDB connection |
| `JWT_SECRET` | Yes | `your_secure_jwt_secret_here_change_in_production` | JWT signing secret |
| `MISTRAL_API_KEY` | Yes | - | Mistral AI API key |
| `PORT` | No | `3000` | Backend port |

---

## Docker Setup

**Services**: backend (Rust on :3000), frontend (Vue on :5173), MongoDB (:27017), Mongo Express (:8081)

**Quick Start**:
```bash
cp .env.example .env
# Edit .env, set MISTRAL_API_KEY
docker-compose up -d
```

**Mongo Express**: `http://localhost:8081` (admin/admin123)

---

## Development Commands

### Backend
```bash
cargo build        # Compile
cargo run          # Run dev server
cargo test         # Run tests
cargo clippy       # Lint
```

### Frontend
```bash
cd frontend
npm install        # Install deps
npm run dev        # Dev server
npm run build      # Production build
npm run lint       # ESLint
```

---

## Important Constraints

1. **No writes without read**: Never edit a file you haven't read in this session.
2. **Respect user constraints**: If user says "no writes", "just analyze", or "plan only" - DO NOT edit files.
3. **Don't remove what wasn't asked**: If fixing X, don't rewrite Y.
4. **Verify changes**: After editing, confirm with test/run/build.
5. **Match style**: 4-space indentation, snake_case for Rust, camelCase for JS.

---

## Common Tasks Reference

### Adding a new API endpoint
1. Add route model in `src/models.rs`
2. Create route handler in `src/routes/<new_file>.rs`
3. Add route in `src/main.rs`
4. Add corresponding service call in `frontend/src/services/`
5. Add store actions in `frontend/src/stores/`

### Adding a new database collection
1. Add struct in `src/models.rs` with `#[derive(Serialize, Deserialize)]`
2. Use `get_collection::<Type>(&state.db, "collection_name")` in routes

### Frontend: Adding a new view
1. Create `.vue` file in `frontend/src/views/`
2. Add route in `frontend/src/router/index.js`
3. Add navigation link in components

---

## Debugging Tips

**Backend**:
- Check logs: `cargo run` shows Axum/Tokio logs
- MongoDB issues: Verify connection string in `.env`
- JWT issues: Check token expiration and secret match

**Frontend**:
- Browser dev tools: Check Network tab for API errors
- CORS: Backend has `CorsLayer::new().allow_origin(Any)` - origin issues are usually frontend config
- Token: Verify JWT is in localStorage and sent in Authorization header

**Docker**:
- Check container logs: `docker-compose logs <service>`
- Verify ports: `docker-compose ps`
- Database connectivity: Test with Mongo Express at `:8081`

---

## File Modification Rules

| Action | Tool | Notes |
|--------|------|-------|
| Read file | `read` | Use offset/limit for large files |
| Search | `grep` | Use regex patterns |
| Edit existing | `edit` | Exact string match required |
| Create new | `write_file` | Fails if file exists |
| Rename/Move | `bash` + `edit` | Use `mv` command |
| Delete | `bash` | Use `rm` command |

---

**Remember**: Be short. Surgical changes. Verify. Match style.
