use std::sync::{Arc, Mutex};

use axum::extract::State;
use chrono::{Days, Local};
use rusqlite::Connection;

pub mod dal;
pub mod routes;

pub type Db = Arc<Mutex<Connection>>;

pub fn seed(State(db): State<Db>) {
    let conn = db.lock().unwrap();

    let today = Local::now();

    for i in 0..=27 {
        let today_minus_i_days = today.checked_sub_days(Days::new(i)).unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO weightentries (amount, created_at) VALUES (?1, ?2)",
            (202.9, format!("{}", today_minus_i_days.format("%Y-%m-%d"))),
        )
        .unwrap();
    }

    for i in 1..=28 {
        let today_minus_i_days = today.checked_sub_days(Days::new(i)).unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO calorieentries (amount, created_at) VALUES (?1, ?2)",
            (2500, format!("{}", today_minus_i_days.format("%Y-%m-%d"))),
        )
        .unwrap();
    }
}
