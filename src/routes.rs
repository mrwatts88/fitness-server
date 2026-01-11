pub mod calories {
    use crate::{Db, dal::calories::CalorieEntry};
    use axum::{
        Json,
        extract::{Path, State},
        response::IntoResponse,
    };
    use chrono::prelude::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateCalorieEntryRequest {
        amount: u32,
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
    use serde::Deserialize;

    use crate::{Db, dal::weight::WeightEntry};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateWeightEntryRequest {
        amount: f64,
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
    use crate::{Db, dal::calories, dal::weight};
    use axum::{Json, extract::State, response::IntoResponse};
    use chrono::{Days, Local};
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Tdee {
        amount: i32,
    }
    #[axum::debug_handler]
    pub async fn get(state: State<Db>) -> impl IntoResponse {
        let today = Local::now();
        let today_minus_1_days = today.checked_sub_days(Days::new(1)).unwrap();
        let today_minus_13_days = today.checked_sub_days(Days::new(13)).unwrap();
        let today_minus_14_days = today.checked_sub_days(Days::new(14)).unwrap();
        let today_minus_27_days = today.checked_sub_days(Days::new(27)).unwrap();
        let today_minus_28_days = today.checked_sub_days(Days::new(28)).unwrap();

        // 1. get [T-28, T-1] records for calorieentries, 28 days of calories in many entries, C
        let c_entries = calories::get_by_date_range(
            state.clone(),
            &format!("{}", today_minus_28_days.format("%Y-%m-%d")),
            &format!("{}", today_minus_1_days.format("%Y-%m-%d")),
        );

        let c_sum: i32 = c_entries.iter().map(|e| e.amount).sum();
        let food_cals_burned = c_sum / 2;
        println!("c_sum: {c_sum}");
        println!("food_cals_burned: {food_cals_burned}");

        // 2. get [T-13, T-0] records for weightentries, 14 days of weights, W2
        let w2_entries = weight::get_by_date_range(
            state.clone(),
            &format!("{}", today_minus_13_days.format("%Y-%m-%d")),
            &format!("{}", today.format("%Y-%m-%d")),
        );

        // 3. get [T-27, T-14] records for weightentries, 14 days of weights, W1
        let w1_entries = weight::get_by_date_range(
            state.clone(),
            &format!("{}", today_minus_27_days.format("%Y-%m-%d")),
            &format!("{}", today_minus_14_days.format("%Y-%m-%d")),
        );

        // 5. avg(W2) = avg weight over last 14 days, AW2
        let w2_sum: f64 = w2_entries.iter().map(|e| e.amount).sum();
        let w2_avg = w2_sum / w2_entries.len() as f64;

        println!("w2_sum: {w2_sum}");
        println!("w2_avg: {w2_avg}");

        // 6. avg(W1) = avg weight over prior 14 days, AW1
        let w1_sum: f64 = w1_entries.iter().map(|e| e.amount).sum();
        let w1_avg = w1_sum / w1_entries.len() as f64;

        println!("w1_sum: {w1_sum}");
        println!("w1_avg: {w1_avg}");

        // 7. AW1 - AW2 = WLoss, loss in pounds in 2 weeks
        let loss = w1_avg - w2_avg;

        println!("loss: {loss}");

        // 8. WLoss * 3500 = FatCalsBurned in 2 weeks
        let fat_cals_burned = loss * 3500.0;

        println!("fat_cals_burned: {fat_cals_burned}");

        // 9. FoodCalsBurned + FatCalsBurned = TotalCalsBurned in 2 weeks
        let total_cals_burned = food_cals_burned + (fat_cals_burned as i32);

        println!("total_cals_burned: {total_cals_burned}");

        // 10. TotalCalsBurned / 14 = TDEE
        Json(Tdee {
            amount: total_cals_burned / 14,
        })
    }
}

pub mod admin {
    use axum::{extract::State, response::IntoResponse};

    use crate::{Db, seed as sd};

    #[axum::debug_handler]
    pub async fn seed(state: State<Db>) -> impl IntoResponse {
        sd(state);
    }
}
