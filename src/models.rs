use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicationPlan {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub medication_name: String,
    pub plan_content: String,
    pub created_at: DateTime,
    pub focus_areas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicationReview {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub plan_id: ObjectId,
    pub date: DateTime,
    pub symptoms: String,
    pub side_effects: String,
    pub notes: String,
    pub rating: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratePlanRequest {
    pub user_id: String,
    pub medication_name: String,
    pub focus_areas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlansQuery {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicationPlanResponse {
    pub id: String,
    pub user_id: String,
    pub medication_name: String,
    pub plan_content: String,
    pub created_at: DateTime,
    pub focus_areas: Vec<String>,
}

impl From<MedicationPlan> for MedicationPlanResponse {
    fn from(plan: MedicationPlan) -> Self {
        Self {
            id: plan.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            user_id: plan.user_id.to_hex(),
            medication_name: plan.medication_name,
            plan_content: plan.plan_content,
            created_at: plan.created_at,
            focus_areas: plan.focus_areas,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicationReviewResponse {
    pub id: String,
    pub user_id: String,
    pub plan_id: String,
    pub date: DateTime,
    pub symptoms: String,
    pub side_effects: String,
    pub notes: String,
    pub rating: i32,
}

impl From<MedicationReview> for MedicationReviewResponse {
    fn from(review: MedicationReview) -> Self {
        Self {
            id: review.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            user_id: review.user_id.to_hex(),
            plan_id: review.plan_id.to_hex(),
            date: review.date,
            symptoms: review.symptoms,
            side_effects: review.side_effects,
            notes: review.notes,
            rating: review.rating,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReviewRequest {
    pub user_id: String,
    pub plan_id: String,
    pub date: String,
    pub symptoms: String,
    pub side_effects: String,
    pub notes: String,
    pub rating: i32,
}
