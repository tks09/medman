use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::{Checkin, CheckinRequest, CheckinResponse};
use crate::AppState;
use axum::{extract::State, Json};
use mongodb::bson::{oid::ObjectId, DateTime};

pub async fn log_checkin(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CheckinRequest>,
) -> Result<Json<CheckinResponse>, AppError> {
    let user_oid = ObjectId::parse_str(&user_id)?;
    let now = DateTime::from(std::time::SystemTime::now());

    let checkin = Checkin {
        id: None,
        user_id: user_oid,
        plan_id: payload.plan_id,
        mood: payload.mood.clone(),
        side_effects: payload.side_effects.clone(),
        note: payload.note,
        date: now,
    };

    let col = state.db.collection::<Checkin>("checkins");
    let result = col.insert_one(checkin).await?;
    let checkin_id = result.inserted_id.as_object_id().unwrap();

    Ok(Json(CheckinResponse {
        id: checkin_id.to_hex(),
        date: chrono::Utc::now().to_rfc3339(),
        mood: payload.mood,
        side_effects: payload.side_effects,
    }))
}
