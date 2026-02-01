use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    routing::{delete, get, put},
};
use fitness_server::{
    Db,
    routes::{admin, calories, quickadd, tdee, weight},
};
use rusqlite::Connection;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let path = "./db.db3";
    let conn = Connection::open(path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS calorieentries (
                id INTEGER PRIMARY KEY,
                amount INTEGER NOT NULL,
                created_at TEXT NOT NULL)", // YYYY-mm-dd HH:MM:SS, local time
        (),
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS weightentries (
                amount INTEGER NOT NULL,
                created_at TEXT PRIMARY KEY)", // YYYY-mm-dd HH:MM:SS, local time
        (),
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS quickaddfoods (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                unit TEXT NOT NULL,
                amount REAL NOT NULL,
                calories INTEGER NOT NULL,
                fat_grams REAL NOT NULL,
                carbs_grams REAL NOT NULL,
                protein_grams REAL NOT NULL,
                sugar_grams REAL NOT NULL,
                created_at TEXT NOT NULL)",
        (),
    )
    .unwrap();

    let arccon: Db = Arc::new(Mutex::new(conn));
    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(Json(HashMap::from([("status", "healthy")]))))
        .route("/calories", get(calories::list).post(calories::create))
        .route("/calories/{id}", delete(calories::delete))
        .route("/weight", get(weight::list).post(weight::create))
        .route("/weight/{id}", delete(weight::delete))
        .route("/tdee", get(tdee::get))
        .route("/quickadd", get(quickadd::list).post(quickadd::create))
        .route(
            "/quickadd/{id}",
            put(quickadd::update).delete(quickadd::delete),
        )
        .route(
            "/quickadd/{id}/consume",
            axum::routing::post(quickadd::consume),
        )
        .route("/seed", get(admin::seed))
        .layer(cors_layer)
        .with_state(arccon);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
