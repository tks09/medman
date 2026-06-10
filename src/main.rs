use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use std::path::PathBuf;

use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

mod auth;
mod database;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let db = database::init_db().await;
    let shared_state = AppState { db: db.clone() };

    log::info!("Starting MedMan");

    let static_dir = PathBuf::from("./static");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    let api_routes = Router::new()
        // Health
        .route("/api/health", get(routes::health::health_check))
        // Auth (new email-based)
        .route("/api/auth/signup", post(routes::auth::signup))
        .route("/api/auth/login", post(routes::auth::login))
        // Plans (new medman-vue format)
        .route("/api/plans", get(routes::plans::get_plans).post(routes::plans::create_plan))
        .route("/api/plans/:id/series", get(routes::plans::get_plan_series))
        // Checkins
        .route("/api/checkins", post(routes::checkins::log_checkin))
        // Insights
        .route("/api/insights", get(routes::insights::get_insights))
        // Legacy medication routes (kept for backward compat)
        .route(
            "/api/medication/plans",
            post(routes::medication::generate_plan).get(routes::medication::get_plans),
        )
        .route("/api/medication/reviews", get(routes::medication::get_reviews))
        .route("/api/medication/reviews", post(routes::medication::create_review))
        .with_state(shared_state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cors);

    let app = Router::new()
        .merge(api_routes)
        .fallback_service(ServeDir::new(&static_dir).append_index_html_on_directories(true));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    log::info!("Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Database,
}
