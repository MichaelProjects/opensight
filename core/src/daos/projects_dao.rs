use diesel::{PgConnection, QueryResult};
use crate::project::Project;


pub struct ProjectIn {
    pub projects_name: String,
}


pub async fn create_project(project_details: Project, conn: &mut PgConnection) -> QueryResult<Project>{

} 