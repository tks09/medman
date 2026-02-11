use crate::error::AppError;
use crate::models::{CreateReviewRequest, GeneratePlanRequest, MedicationPlan, MedicationReview};
use crate::AppState;
use axum::{extract::State, Json};
use chrono::DateTime as ChronoDateTime;
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use reqwest::Client;
use serde_json::json;

pub async fn generate_plan(
    State(state): State<AppState>,
    Json(payload): Json<GeneratePlanRequest>,
) -> Result<Json<MedicationPlan>, AppError> {
    // Generate plan using Mistral AI API
    let plan_content = generate_ai_plan(&payload.medication_name, &payload.focus_areas).await?;

    let user_id = ObjectId::parse_str(&payload.user_id)?;

    let plan = MedicationPlan {
        id: None,
        user_id,
        medication_name: payload.medication_name,
        plan_content,
        created_at: DateTime::from(std::time::SystemTime::now()),
        focus_areas: payload.focus_areas,
    };

    let plans_collection = state.db.collection::<MedicationPlan>("medication_plans");
    let insert_result = plans_collection.insert_one(plan.clone()).await?;

    let plan_id = insert_result.inserted_id.as_object_id().unwrap();

    let mut plan_with_id = plan;
    plan_with_id.id = Some(plan_id);

    Ok(Json(plan_with_id))
}

pub async fn get_reviews(
    State(state): State<AppState>,
    // Add user_id parameter extraction here
) -> Result<Json<Vec<MedicationReview>>, AppError> {
    // TODO: Implement proper user authentication and extract user_id
    let user_id = ObjectId::parse_str("65d0b2b3d3b0b3d3b0b3d3b0")?; // Placeholder

    let reviews_collection = state
        .db
        .collection::<MedicationReview>("medication_reviews");
    let cursor = reviews_collection.find(doc! {"user_id": user_id}).await?;

    let reviews = cursor.try_collect::<Vec<_>>().await?;

    Ok(Json(reviews))
}

pub async fn create_review(
    State(state): State<AppState>,
    Json(payload): Json<CreateReviewRequest>,
) -> Result<Json<MedicationReview>, AppError> {
    let user_id = ObjectId::parse_str(&payload.user_id)?;
    let plan_id = ObjectId::parse_str(&payload.plan_id)?;
    let date = ChronoDateTime::parse_from_rfc3339(&payload.date)
        .map_err(|e| AppError::ValidationError(format!("Invalid date format: {}", e)))?;

    let review = MedicationReview {
        id: None,
        user_id,
        plan_id,
        date: DateTime::from(std::time::SystemTime::from(date)),
        symptoms: payload.symptoms,
        side_effects: payload.side_effects,
        notes: payload.notes,
        rating: payload.rating,
    };

    let reviews_collection = state
        .db
        .collection::<MedicationReview>("medication_reviews");
    let insert_result = reviews_collection.insert_one(review.clone()).await?;

    let review_id = insert_result.inserted_id.as_object_id().unwrap();

    let mut review_with_id = review;
    review_with_id.id = Some(review_id);

    Ok(Json(review_with_id))
}

async fn generate_ai_plan(
    medication_name: &str,
    focus_areas: &[String],
) -> Result<String, AppError> {
    let mistral_api_key = std::env::var("MISTRAL_API_KEY")
        .map_err(|_| AppError::ValidationError("MISTRAL_API_KEY not set".to_string()))?;

    let client = Client::new();

    let prompt = format!(
        "Generate a comprehensive medication review plan for someone taking {}. \n\nFocus on these areas: {}. \n\nInclude daily tracking questions, potential side effects to watch for, and any specific considerations related to the focus areas. \n\nProvide a structured plan that can be used for daily reviews.",
        medication_name,
        focus_areas.join(", ")
    );

    let response = client
        .post("https://api.mistral.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", mistral_api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "mistral-tiny",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.7
        }))
        .send()
        .await
        .map_err(|e| AppError::ValidationError(format!("Failed to call Mistral API: {}", e)))?;

    let body = response.json::<serde_json::Value>().await.map_err(|e| {
        AppError::ValidationError(format!("Failed to parse Mistral API response: {}", e))
    })?;

    let plan_content = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Failed to generate plan")
        .to_string();

    Ok(plan_content)
}
