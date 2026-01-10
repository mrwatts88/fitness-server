use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub mod routes;

pub type Db = Arc<Mutex<Connection>>;
