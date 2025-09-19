use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use crate::application::user_service::UserService;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler<R>(State(service): State<UserService<R>>, Json(payload): Json<LoginRequest>) -> impl IntoResponse
where
    R: crate::domain::repository::UserRepository + Send + Sync + Clone + 'static,
{
    match service.login(&payload.email, &payload.password).await {
        Ok(token) => (StatusCode::OK, Json(serde_json::json!({"token": token}))),
        Err(_) => (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid credentials"}))),
    }
}
