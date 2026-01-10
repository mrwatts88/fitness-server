use axum::{Router, routing::get};
use fitness_server::routes::{calories, tdee, weight};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get("healthy"))
        .route("/calories", get(calories::list))
        .route("/weight", get(weight::list))
        .route("/tdee", get(tdee::get));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
