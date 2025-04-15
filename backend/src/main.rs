use axum::{
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct AuthRequest {
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    success: bool,
}

#[tokio::main]
async fn main() {
    // Configure CORS inside the main function
    let cors = CorsLayer::new()
        .allow_origin(Any) // Or use your specific origin
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/auth", post(authenticate))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn authenticate(Json(auth_request): Json<AuthRequest>) -> Json<AuthResponse> {
    let stored_hash = "d9a26e01e912441831cdb46dd3d9a269"; //server side stored hash password for --Rust-tutorial
    let password_hash = hash_password(auth_request.password);
    let success = stored_hash == password_hash;
    Json(AuthResponse { success })
}

fn hash_password(password: String) -> String {
    let hashed_pass = md5::compute(password);
    format!("{:x}", hashed_pass)
}

