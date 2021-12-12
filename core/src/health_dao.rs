use diesel::{PgConnection, QueryResult, RunQueryDsl};
use std::error;
use diesel;

pub fn get_tables(connection: &mut PgConnection) -> Result<Vec<String>, Box<dyn error::Error>> {
    let response = diesel::sql_query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'").load(&connection)?;
    Ok(response);
}