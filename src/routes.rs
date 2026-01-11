pub mod calories {
    use crate::Db;

    use axum::{
        Json,
        extract::{Path, State},
        response::IntoResponse,
    };
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateCalorieEntryRequest {
        amount: u32,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct CalorieEntry {
        id: u32,
        amount: u32,
        created_at: String,
    }

    /**
     * Get all calorie entries from today local time, most recent first.
     */
    #[axum::debug_handler]
    pub async fn list(State(db): State<Db>) -> impl IntoResponse {
        let conn = db.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT id, amount, created_at FROM calorieentries WHERE created_at >= :yesterday ORDER BY created_at DESC",
            )
            .unwrap();

        let now_local = Local::now();
        let start_of_today = now_local
            .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .unwrap();
        let formatted = format!("{}", start_of_today.format("%Y-%m-%d %H:%M:%S"));

        let results = stmt
            .query_map(&[(":yesterday", &formatted)], |row| {
                Ok(CalorieEntry {
                    id: row.get(0).unwrap(),
                    amount: row.get(1).unwrap(),
                    created_at: row.get(2).unwrap(),
                })
            })
            .unwrap();

        let entries = results.map(|result| result.unwrap());

        Json(entries.collect::<Vec<CalorieEntry>>())
    }

    #[axum::debug_handler]
    pub async fn create(
        State(db): State<Db>,
        Json(input): Json<CreateCalorieEntryRequest>,
    ) -> impl IntoResponse {
        let conn = db.lock().unwrap();

        let now_local = Local::now();
        let formatted = format!("{}", now_local.format("%Y-%m-%d %H:%M:%S"));
        conn.execute(
            "INSERT INTO calorieentries (amount, created_at) VALUES (?1, ?2)",
            (input.amount, &formatted),
        )
        .unwrap();
    }

    #[axum::debug_handler]
    pub async fn delete(State(db): State<Db>, Path(id): Path<i32>) -> impl IntoResponse {
        let conn = db.lock().unwrap();

        conn.execute("DELETE FROM calorieentries WHERE id = ?1", [id])
            .unwrap();
    }
}

pub mod weight {
    use axum::{
        Json,
        extract::{Path, State},
        response::IntoResponse,
    };
    use chrono::Local;
    use serde::{Deserialize, Serialize};

    use crate::Db;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateWeightEntryRequest {
        amount: f64,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct WeightEntry {
        amount: f64,
        created_at: String,
    }

    /**
     * Get up to 90 of the most recent weight entries, most recent first.
     * Used to show historical weight and graph, as well as current weight.
     */
    #[axum::debug_handler]
    pub async fn list(State(db): State<Db>) -> impl IntoResponse {
        let conn = db.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT amount, created_at FROM weightentries ORDER BY created_at DESC LIMIT 90",
            )
            .unwrap();

        let results = stmt
            .query_map([], |row| {
                Ok(WeightEntry {
                    amount: row.get(0).unwrap(),
                    created_at: row.get(1).unwrap(),
                })
            })
            .unwrap();

        let entries = results.map(|result| result.unwrap());

        Json(entries.collect::<Vec<WeightEntry>>())
    }

    #[axum::debug_handler]
    pub async fn create(
        State(db): State<Db>,
        Json(input): Json<CreateWeightEntryRequest>,
    ) -> impl IntoResponse {
        let conn = db.lock().unwrap();

        let now_local = Local::now();
        let formatted = format!("{}", now_local.format("%Y-%m-%d"));
        conn.execute(
            "INSERT OR REPLACE INTO weightentries (amount, created_at) VALUES (?1, ?2)",
            (input.amount, &formatted),
        )
        .unwrap();
    }

    #[axum::debug_handler]
    pub async fn delete(State(db): State<Db>, Path(date): Path<String>) -> impl IntoResponse {
        let conn = db.lock().unwrap();

        conn.execute("DELETE FROM weightentries WHERE created_at = ?1", [date])
            .unwrap();
    }
}

pub mod tdee {
    use axum::{Json, response::IntoResponse};
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Tdee {
        amount: u32,
    }
    #[axum::debug_handler]
    pub async fn get() -> impl IntoResponse {
        Json(Tdee { amount: 2750 })
        // todo: implement
    }
}
