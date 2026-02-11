use crate::auth::{create_jwt, hash_password, verify_password};
use crate::error::AppError;
use crate::models::{AuthRequest, AuthResponse, User};
use crate::AppState;
use axum::{extract::State, Json};
use mongodb::bson::{doc, DateTime};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Check if user already exists
    let users_collection = state.db.collection::<User>("users");
    let existing_user: Option<User> = users_collection
        .find_one(doc! {"username": &payload.username})
        .await?;

    if existing_user.is_some() {
        return Err(AppError::ValidationError(
            "Username already exists".to_string(),
        ));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Create user
    let user = User {
        id: None,
        username: payload.username.clone(),
        email: format!("{}@example.com", payload.username), // Simple email for now
        password_hash,
        created_at: DateTime::from(std::time::SystemTime::now()),
    };

    let insert_result = users_collection.insert_one(user).await?;
    let user_id = insert_result.inserted_id.as_object_id().unwrap();

    // Create JWT
    let token = create_jwt(&user_id.to_hex())?;

    Ok(Json(AuthResponse {
        token,
        user_id: user_id.to_hex(),
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let users_collection = state.db.collection::<User>("users");

    let user = users_collection
        .find_one(doc! {"username": &payload.username})
        .await?
        .ok_or(AppError::AuthError("Invalid credentials".to_string()))?;

    // Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::AuthError("Invalid credentials".to_string()));
    }

    // Create JWT
    let user_id = user.id.unwrap();
    let token = create_jwt(&user_id.to_hex())?;

    Ok(Json(AuthResponse {
        token,
        user_id: user_id.to_hex(),
    }))
}
