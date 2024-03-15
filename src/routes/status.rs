use rocket::serde::json::Json;

use crate::models::response::MessageResponse;

#[get("/health")]
pub fn health() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "healthy".to_string(),
    })
}
