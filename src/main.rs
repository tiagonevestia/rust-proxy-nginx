use dotenv::dotenv;
use std::env;

use axum::{routing::get, Json, Router};

async fn handler() -> Json<String> {
    dotenv().ok();
    let instance_name = env::var("INSTANCE_NAME").expect("INSTANCE_NAME must be set");
    Json::from(instance_name)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port");

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("Server failed");
}
