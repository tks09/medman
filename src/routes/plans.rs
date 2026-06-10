use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::{CreatePlanRequest, Plan, PlanResponse};
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, DateTime};

pub async fn get_plans(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<PlanResponse>>, AppError> {
    let user_oid = ObjectId::parse_str(&user_id)?;
    let col = state.db.collection::<Plan>("plans");
    let cursor = col.find(doc! {"user_id": user_oid}).await?;
    let plans: Vec<PlanResponse> = cursor
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(plans))
}

pub async fn create_plan(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreatePlanRequest>,
) -> Result<Json<PlanResponse>, AppError> {
    let user_oid = ObjectId::parse_str(&user_id)?;
    let name = payload.name;
    let dose = payload.dose.unwrap_or_else(|| "As needed".to_string());
    let focus = payload.focus;

    let plan = Plan {
        id: None,
        user_id: user_oid,
        name: name.clone(),
        dose: dose.clone(),
        focus: focus.clone(),
        adherence: 0,
        created_at: DateTime::from(std::time::SystemTime::now()),
    };

    let col = state.db.collection::<Plan>("plans");
    let result = col.insert_one(plan).await?;
    let plan_id = result.inserted_id.as_object_id().unwrap();

    Ok(Json(PlanResponse {
        id: plan_id.to_hex(),
        name,
        dose,
        focus,
        adherence: 0,
    }))
}

pub async fn get_plan_series(
    State(_state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Path(_plan_id): Path<String>,
) -> Result<Json<Vec<i32>>, AppError> {
    Ok(Json(vec![3, 2, 3, 4, 3, 4, 5, 4, 4, 5, 4, 5, 5]))
}
