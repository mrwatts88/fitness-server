use serde::{Deserialize, Serialize};

pub mod calories {
    use crate::{
        Db,
        routes::{CalorieEntry, CreateCalorieEntryRequest},
    };

    use axum::{
        Json,
        extract::{Path, State},
        response::IntoResponse,
    };

    #[axum::debug_handler]
    pub async fn list(State(db): State<Db>) -> impl IntoResponse {
        let conn = db.lock().unwrap();

        let mut stmt = conn
            .prepare("SELECT id, amount FROM calorieentries")
            .unwrap();

        let results = stmt
            .query_map([], |row| {
                Ok(CalorieEntry {
                    id: row.get(0).unwrap(),
                    amount: row.get(1).unwrap(),
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

        conn.execute(
            "INSERT INTO calorieentries (amount) VALUES (?1)",
            [input.amount],
        )
        .unwrap();
    }

    #[axum::debug_handler]
    pub async fn delete(Path(id): Path<i32>) -> impl IntoResponse {
        println!("Deleting CalorieEntry with ID: {:?}", id);
        // todo: delete data
    }
}

pub mod weight {
    use axum::{
        Json,
        extract::{Path, State},
        response::IntoResponse,
    };

    use crate::{
        Db,
        routes::{CreateWeightEntryRequest, WeightEntry},
    };

    #[axum::debug_handler]
    pub async fn list(State(db): State<Db>) -> impl IntoResponse {
        let conn = db.lock().unwrap();

        let mut stmt = conn
            .prepare("SELECT id, amount FROM weightentries")
            .unwrap();

        let results = stmt
            .query_map([], |row| {
                Ok(WeightEntry {
                    id: row.get(0).unwrap(),
                    amount: row.get(1).unwrap(),
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

        conn.execute(
            "INSERT INTO weightentries (amount) VALUES (?1)",
            [input.amount],
        )
        .unwrap();
    }

    #[axum::debug_handler]
    pub async fn delete(Path(id): Path<i32>) -> impl IntoResponse {
        println!("Deleting WeightEntry with ID: {:?}", id);
        // todo: delete data
    }
}

pub mod tdee {
    use axum::{Json, response::IntoResponse};

    use crate::routes::Tdee;

    #[axum::debug_handler]
    pub async fn get() -> impl IntoResponse {
        Json(Tdee { amount: 2750 })
        // todo: implement
    }
}

#[derive(Debug, Serialize)]
pub struct CalorieEntry {
    id: u32,
    amount: u32,
}

#[derive(Debug, Deserialize)]
pub struct CreateCalorieEntryRequest {
    amount: u32,
}

#[derive(Debug, Serialize)]
pub struct WeightEntry {
    id: u32,
    amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateWeightEntryRequest {
    amount: f64,
}

#[derive(Debug, Serialize)]
pub struct Tdee {
    amount: u32,
}
