use serde::Serialize;

pub mod calories {
    use axum::{Json, response::IntoResponse};

    use crate::routes::CalorieEntry;

    pub async fn list() -> impl IntoResponse {
        Json(vec![
            CalorieEntry { amount: 755 },
            CalorieEntry { amount: 240 },
        ])
    }
}

pub mod weight {
    use axum::{Json, response::IntoResponse};

    use crate::routes::WeightEntry;

    pub async fn list() -> impl IntoResponse {
        Json(vec![
            WeightEntry { amount: 201.4 },
            WeightEntry { amount: 202.3 },
        ])
    }
}

pub mod tdee {
    use axum::{Json, response::IntoResponse};

    use crate::routes::Tdee;

    pub async fn get() -> impl IntoResponse {
        Json(Tdee { amount: 2750 })
    }
}

#[derive(Serialize)]
pub struct CalorieEntry {
    amount: u32,
}

#[derive(Serialize)]
pub struct WeightEntry {
    amount: f64,
}

#[derive(Serialize)]
pub struct Tdee {
    amount: u32,
}
