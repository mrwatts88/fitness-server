pub mod weight {
    use crate::Db;
    use axum::extract::State;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WeightEntry {
        pub amount: f64,
        pub created_at: String,
    }

    pub fn get_by_date_range(State(db): State<Db>, from: &str, to: &str) -> Vec<WeightEntry> {
        let conn = db.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT amount, created_at FROM weightentries WHERE created_at >= :from AND created_at <= :to ORDER BY created_at DESC",
            )
            .unwrap();

        let results = stmt
            .query_map(&[(":from", from), (":to", to)], |row| {
                Ok(WeightEntry {
                    amount: row.get(0).unwrap(),
                    created_at: row.get(1).unwrap(),
                })
            })
            .unwrap();

        let iter = results.map(|r| r.unwrap());

        iter.collect::<Vec<WeightEntry>>()
    }
}

pub mod calories {
    use crate::Db;
    use axum::extract::State;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CalorieEntry {
        pub id: u32,
        pub amount: u32,
        pub created_at: String,
    }

    pub fn get_by_date_range(State(db): State<Db>, from: &str, to: &str) -> Vec<CalorieEntry> {
        let conn = db.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT id, amount, created_at FROM calorieentries WHERE created_at >= :from AND created_at <= :to ORDER BY created_at DESC",
            )
            .unwrap();

        let results = stmt
            .query_map(&[(":from", from), (":to", to)], |row| {
                Ok(CalorieEntry {
                    id: row.get(0).unwrap(),
                    amount: row.get(1).unwrap(),
                    created_at: row.get(2).unwrap(),
                })
            })
            .unwrap();

        let iter = results.map(|r| r.unwrap());

        iter.collect::<Vec<CalorieEntry>>()
    }
}
