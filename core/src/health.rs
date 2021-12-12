use crate::health_dao{het_tables};
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Health {
    pub status_code: u32,
    pub connection: bool,
    pub tables_present: bool
}
impl Health {
    fn new(status_code: u32, connection: bool, tables_present: bool) -> Health {
        Health {
            status_code,
            connection,
            tables_present
}
    }


pub async fn check_health(conn: &mut PgConnection) -> Health {
    let response_data = conn.run(|c|  get_tables(c)).await;
}