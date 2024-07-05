use axum::{routing::post, Router};
use real_time_app::create_user;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/auth/register", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();    
}

