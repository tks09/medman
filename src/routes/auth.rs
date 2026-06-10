use crate::auth::{create_jwt, hash_password, verify_password};
use crate::error::AppError;
use crate::models::{AuthResponse, LoginRequest, SignupRequest, User};
use crate::AppState;
use axum::{extract::State, Json};
use mongodb::bson::{doc, DateTime};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let users = state.db.collection::<User>("users");

    let existing = users.find_one(doc! {"email": &payload.email}).await?;
    if existing.is_some() {
        return Err(AppError::ValidationError("Email already registered".to_string()));
    }

    let password_hash = hash_password(&payload.password)?;

    let user = User {
        id: None,
        email: payload.email.clone(),
        name: payload.name.clone(),
        password_hash,
        created_at: DateTime::from(std::time::SystemTime::now()),
    };

    let result = users.insert_one(user).await?;
    let user_id = result.inserted_id.as_object_id().unwrap();
    let token = create_jwt(&user_id.to_hex())?;

    Ok(Json(AuthResponse {
        id: user_id.to_hex(),
        name: payload.name,
        email: payload.email,
        token,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let users = state.db.collection::<User>("users");

    let user = users
        .find_one(doc! {"email": &payload.email})
        .await?
        .ok_or_else(|| AppError::AuthError("Invalid credentials".to_string()))?;

    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::AuthError("Invalid credentials".to_string()));
    }

    let user_id = user.id.unwrap();
    let token = create_jwt(&user_id.to_hex())?;

    Ok(Json(AuthResponse {
        id: user_id.to_hex(),
        name: user.name,
        email: user.email,
        token,
    }))
}
