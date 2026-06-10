use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

/* ---- User ---- */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub created_at: DateTime,
}

/* ---- Auth ---- */

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub token: String,
}

/* ---- Plan ---- */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plan {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub name: String,
    pub dose: String,
    pub focus: Vec<String>,
    pub adherence: i32,
    pub created_at: DateTime,
}

#[derive(Debug, Serialize)]
pub struct PlanResponse {
    pub id: String,
    pub name: String,
    pub dose: String,
    pub focus: Vec<String>,
    pub adherence: i32,
}

impl From<Plan> for PlanResponse {
    fn from(p: Plan) -> Self {
        Self {
            id: p.id.map(|o| o.to_hex()).unwrap_or_default(),
            name: p.name,
            dose: p.dose,
            focus: p.focus,
            adherence: p.adherence,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub name: String,
    pub dose: Option<String>,
    pub focus: Vec<String>,
}

/* ---- Checkin ---- */

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkin {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub plan_id: Option<String>,
    pub mood: String,
    pub side_effects: Vec<String>,
    pub note: Option<String>,
    pub date: DateTime,
}

#[derive(Debug, Deserialize)]
pub struct CheckinRequest {
    pub mood: String,
    #[serde(rename = "sideEffects")]
    pub side_effects: Vec<String>,
    pub note: Option<String>,
    #[serde(rename = "planId")]
    pub plan_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckinResponse {
    pub id: String,
    pub date: String,
    pub mood: String,
    pub side_effects: Vec<String>,
}

/* ---- Insight ---- */

#[derive(Debug, Serialize)]
pub struct Insight {
    pub id: String,
    pub icon: String,
    pub tone: String,
    pub title: String,
    pub body: String,
}

/* ---- Legacy (kept for existing /api/medication routes) ---- */

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

#[derive(Debug, Deserialize)]
pub struct GeneratePlanRequest {
    pub user_id: String,
    pub medication_name: String,
    pub focus_areas: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetPlansQuery {
    pub user_id: String,
}

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub user_id: String,
    pub plan_id: String,
    pub date: String,
    pub symptoms: String,
    pub side_effects: String,
    pub notes: String,
    pub rating: i32,
}
