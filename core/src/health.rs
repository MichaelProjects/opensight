use crate::{db::DatabaseConnection};
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Health {
    pub status_code: u32,
    pub connection: bool,
}
impl Health {
    pub fn new(status_code: u32, connection: bool) -> Health {
        Health {
            status_code,
            connection,
        }
    }
}
