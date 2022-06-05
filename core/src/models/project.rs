use std::error::Error;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{schema::projects, daos::projects_dao::ProjectIn, db::DatabaseConnection};
use crate::daos::projects_dao::{create_project, get_all_projects};

#[derive(Serialize, Clone, Debug, Hash, Queryable, Insertable)]
#[table_name = "projects"]
pub struct Project {
    pub id: String,
    pub projects_name: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime
}
impl Project {
    pub fn new(data: ProjectIn) -> Project {
        let project_id:String = Uuid::new_v4().to_string();
        let get_time = Utc::now().naive_utc();
        let mut project = Project {
            id: project_id,
            projects_name: data.projects_name,
            created: get_time,
            updated: get_time
        };
        return project
    }
    pub async fn insert(project: Project, conn: &DatabaseConnection) -> Result<Project, Box<dyn Error>> {
        let response= conn.run(|c| create_project(project, c)).await?;
        Ok(response)
    }
    
    pub async fn select_all(conn: &DatabaseConnection) -> Result<Vec<Project>, Box<dyn Error>>{
        let response = conn.run(|c| get_all_projects(c)).await?;
        Ok(response)
    }
}