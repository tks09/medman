# Medicine Manager

A web application for tracking medication effects and side effects using AI-generated review plans.

## Quick Start with Docker

The easiest way to get started is with our setup script:

```bash
./setup.sh
```

Or manually:

```bash
# Copy environment file and set your Mistral AI API key
cp backend/.env.example backend/.env

# Edit backend/.env and set MISTRAL_API_KEY

# Start all services
docker-compose up -d

# Access the application
- Frontend: http://localhost:5173
- MongoDB UI: http://localhost:8081 (admin/admin123)
```

## Features

- **User Authentication**: Secure login and registration system
- **AI-Powered Plan Generation**: Generate comprehensive medication review plans using Mistral AI
- **Daily Tracking**: Record symptoms, side effects, and progress
- **Personalized Insights**: Focus on specific areas of concern (e.g., digestive functions, mood, sleep)
- **Dashboard**: View all your medication plans and reviews in one place
- **Docker Support**: Complete containerized setup with MongoDB and Mongo Express UI
- **Mongo Express UI**: Web-based MongoDB administration interface

## Technology Stack

### Backend
- **Language**: Rust
- **Framework**: Axum + Tokio
- **Database**: MongoDB
- **Authentication**: JWT with bcrypt
- **AI Integration**: Mistral AI API

### Frontend
- **Framework**: Vue.js 3
- **State Management**: Pinia
- **Routing**: Vue Router
- **Styling**: Tailwind CSS
- **HTTP Client**: Axum

## Project Structure

```
medman/
├── src/                  # Rust backend source code
├── Cargo.toml            # Rust dependencies
├── Cargo.lock            # Rust dependency lock file
├── Dockerfile            # Backend Dockerfile
├── .env.example          # Environment variables template
│
├── frontend/             # Vue.js frontend
│   ├── src/              # Source code
│   ├── public/           # Public assets
│   ├── package.json      # JavaScript dependencies
│   ├── Dockerfile        # Frontend Dockerfile
│   └── vite.config.js     # Vite configuration
│
├── .gitignore            # Git ignore rules
├── Purpose.md            # Original project requirements
├── README.md             # This file
├── docker-compose.yml    # Docker Compose configuration
└── setup.sh             # Quick setup script
```

## Setup Instructions

### Prerequisites

- Docker and Docker Compose (for containerized setup)
- OR Rust (latest stable version) + Node.js (v18+) + MongoDB (v6+) for manual setup
- Mistral AI API key

### Docker Setup (Recommended)

The easiest way to run the application is using Docker Compose:

1. Copy the environment file:
   ```bash
   cp backend/.env.example backend/.env
   ```

2. Edit `backend/.env` and set your Mistral AI API key:
   ```env
   MISTRAL_API_KEY=your_mistral_api_key_here
   ```

3. Start all services:
   ```bash
   docker-compose up -d
   ```

4. Access the services:
   - Frontend: `http://localhost:5173`
   - Backend API: `http://localhost:3000`
   - MongoDB UI (Mongo Express): `http://localhost:8081` (username: admin, password: admin123)
   - MongoDB: `mongodb://localhost:27017`

5. To stop the services:
   ```bash
   docker-compose down
   ```

### Manual Setup

#### Backend Setup

1. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` and set your configuration:
   ```env
   MONGODB_URI=mongodb://localhost:27017
   JWT_SECRET=your_secure_jwt_secret
   MISTRAL_API_KEY=your_mistral_api_key
   ```

3. Install Rust dependencies:
   ```bash
   cargo build
   ```

4. Run the backend server:
   ```bash
   cargo run
   ```

The backend will be available at `http://localhost:3000`

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```

2. Install JavaScript dependencies:
   ```bash
   npm install
   ```

3. Run the development server:
   ```bash
   npm run dev
   ```

The frontend will be available at `http://localhost:5173`

### Running in Production

#### Backend
```bash
cd backend
cargo build --release
./target/release/medman-backend
```

#### Frontend
```bash
cd frontend
npm run build
npm run preview
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login and get JWT token

### Medication Plans
- `POST /api/medication/plans` - Generate a new medication plan
- `GET /api/medication/plans` - Get user's medication plans (TODO)

### Reviews
- `GET /api/medication/reviews` - Get user's medication reviews
- `POST /api/medication/reviews` - Create a new medication review

### Health Check
- `GET /api/health` - Check server health

## Environment Variables

### Backend Variables

| Variable | Description | Required | Docker Default |
|----------|-------------|----------|---------------|
| `MONGODB_URI` | MongoDB connection string | Yes | `mongodb://root:example@mongodb:27017/medman?authSource=admin` |
| `JWT_SECRET` | Secret for JWT token signing | Yes | `your_secure_jwt_secret_here_change_in_production` |
| `MISTRAL_API_KEY` | Mistral AI API key | Yes | (must be set) |
| `PORT` | Server port (default: 3000) | No | `3000` |

**Note**: Environment variables are now in the root `.env` file instead of `backend/.env`

### MongoDB Credentials (Docker)

- **Root Username**: `root`
- **Root Password**: `example`
- **Database**: `medman`
- **Auth Source**: `admin`

### Mongo Express UI Credentials

- **URL**: `http://localhost:8081`
- **Username**: `admin`
- **Password**: `admin123`

## Docker Services

The Docker Compose setup includes:

- **Medicine Manager Backend**: Rust/Axum API server
- **Medicine Manager Frontend**: Vue.js web application
- **MongoDB**: Database for storing user data and medication reviews
- **Mongo Express**: Web-based MongoDB administration interface

## Accessing Mongo Express

Mongo Express provides a user-friendly web interface for MongoDB:

- **URL**: `http://localhost:8081`
- **Username**: `admin`
- **Password**: `admin123`

Use Mongo Express to:
- Browse collections and documents
- Run queries and aggregations
- Manage indexes
- View database statistics

## Development Notes

### Backend

The Rust backend uses:
- Axum for web routing
- Tokio for async runtime
- MongoDB for data storage
- JSON Web Tokens for authentication
- Reqwest for Mistral AI API calls
- State management with Axum's `State` extractor

### Fix for "Missing request extension" Error

If you encounter the error "Missing request extension: Extension of type `AppState` was not found", this was fixed by:

1. **Changing from `Extension` to `State`**: Updated all route handlers to use `State<AppState>` instead of `Extension<AppState>`
2. **Using `with_state()` instead of `layer(Extension())`**: Updated the router configuration in `main.rs`
3. **Removing `Arc` wrapping**: The `State` extractor handles sharing automatically, so `Arc` is no longer needed for the shared state

The corrected pattern:
```rust
// In main.rs
let shared_state = AppState { db: db.clone() };
let app = Router::new()
    .route("/api/auth/register", post(routes::auth::register))
    .with_state(shared_state)  // Instead of .layer(Extension(shared_state))

// In route handlers
pub async fn register(
    State(state): State<AppState>,  // Instead of Extension(state): Extension<AppState>
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError>
```

### Frontend

The Vue.js frontend uses:
- Vue Router for navigation
- Pinia for state management
- Axios for API calls
- Tailwind CSS for styling
- Vite for build tooling

### Authentication Flow

1. User registers or logs in
2. Backend returns JWT token
3. Frontend stores token in localStorage
4. Token is sent with each API request via Authorization header
5. Protected routes check for valid token

## Future Enhancements

- Add email verification
- Implement password reset functionality
- Add more detailed medication tracking metrics
- Implement data export functionality
- Add charts and visualizations for trends
- Implement reminders and notifications

## License

This project is licensed under the MIT License.