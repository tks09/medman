use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};

mod auth;
mod database;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    // Initialize database
    let db = database::init_db().await;

    // Create shared state
    let shared_state = AppState { db: db.clone() };
    log::info!("Starting MedMan");
    // Build our application with routes
    let app = Router::new()
        .route("/api/health", get(routes::health::health_check))
        .route("/api/auth/register", post(routes::auth::register))
        .route("/api/auth/login", post(routes::auth::login))
        .route(
            "/api/medication/plans",
            post(routes::medication::generate_plan),
        )
        .route(
            "/api/medication/reviews",
            get(routes::medication::get_reviews),
        )
        .route(
            "/api/medication/reviews",
            post(routes::medication::create_review),
        )
        .with_state(shared_state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        )
        .fallback(routes::not_found::handler_404);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Database,
}
