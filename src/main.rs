use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    routing::{delete, get},
};
use fitness_server::{
    Db,
    routes::{calories, tdee, weight},
};
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let path = "./db.db3";
    let conn = Connection::open(path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS calorieentries (
                id INTEGER PRIMARY KEY,
                amount INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP)",
        (),
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS weightentries (
                id INTEGER PRIMARY KEY,
                amount INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP)",
        (),
    )
    .unwrap();

    let arccon: Db = Arc::new(Mutex::new(conn));

    let app = Router::new()
        .route("/health", get(Json(HashMap::from([("status", "healthy")]))))
        .route("/calories", get(calories::list).post(calories::create))
        .route("/calories/{id}", delete(calories::delete))
        .route("/weight", get(weight::list).post(weight::create))
        .route("/weight/{id}", delete(weight::delete))
        .route("/tdee", get(tdee::get))
        .with_state(arccon);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
