use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::Insight;
use axum::Json;

pub async fn get_insights(
    AuthUser(_user_id): AuthUser,
) -> Result<Json<Vec<Insight>>, AppError> {
    Ok(Json(vec![
        Insight {
            id: "i_1".to_string(),
            icon: "spark".to_string(),
            tone: "ai".to_string(),
            title: "Mood is trending up".to_string(),
            body: "Your check-in scores have improved over the past week. Mornings after 7h+ sleep score highest.".to_string(),
        },
        Insight {
            id: "i_2".to_string(),
            icon: "activity".to_string(),
            tone: "accent".to_string(),
            title: "Fewer side-effect flags".to_string(),
            body: "Side-effects dropped on days you logged a meal before your dose.".to_string(),
        },
        Insight {
            id: "i_3".to_string(),
            icon: "clock".to_string(),
            tone: "accent".to_string(),
            title: "Consistent timing helps".to_string(),
            body: "Energy scores are highest when you take your medication within an hour of waking.".to_string(),
        },
        Insight {
            id: "i_4".to_string(),
            icon: "heart".to_string(),
            tone: "ai".to_string(),
            title: "Talk to your doctor".to_string(),
            body: "You've logged dizziness multiple times. Worth mentioning at your next appointment.".to_string(),
        },
    ]))
}
