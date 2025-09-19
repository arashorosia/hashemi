use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router, Server,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Simple demo login handler without database
async fn demo_login_handler(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    println!("Login attempt: {}", payload.email);
    
    // Phoenix TechWorks authentication - accept admin@phoenixtechworks.com with password "Phoenix2025!"
    if payload.email == "admin@phoenixtechworks.com" && payload.password == "Phoenix2025!" {
        Ok(Json(LoginResponse {
            token: "ptw-auth-token-2025".to_string(),
            message: "Access granted to Phoenix TechWorks systems".to_string(),
        }))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid credentials or unauthorized access".to_string(),
            }),
        ))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Starting Phoenix TechWorks Authentication Server...");
    println!("📡 Server will be available at: http://localhost:3000");
    println!("🔑 Access credentials: admin@phoenixtechworks.com / Phoenix2025!");
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/login", post(demo_login_handler))
        .route("/health", get(|| async { "OK" }))
        .layer(cors);

    println!("✅ Server listening on port 3000");
    
    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}