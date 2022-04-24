use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use super::schema::projects;

#[derive(Serialize, Clone, Debug, Hash, Queryable, Insertable)]
#[table_name = "projects"]
pub struct Project {
    pub id: String,
    pub projects_name: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime
}