use diesel::{PgConnection, QueryResult, RunQueryDsl};
use serde::Deserialize;
use crate::utils::dao_error::DaoError;
use crate::models::project::Project;
use crate::schema::projects;
use diesel::prelude::*;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct ProjectIn {
    pub projects_name: String,
}


pub  fn create_project(project_details: Project, conn: &mut PgConnection) -> QueryResult<Project>{
    let response: QueryResult<Project> = diesel::insert_into(projects::table)
    .values(project_details)
    .get_result(conn);
    response
}


pub fn get_project(id: String, conn: &mut PgConnection) -> Result<Project, diesel::result::Error>{
    let response: QueryResult<Project> = projects::table.filter(projects::id.eq(id)).get_result(conn);
    response
}


pub fn get_all_projects(conn: &PgConnection) -> Result<Vec<Project>, diesel::result::Error> {
    let response: QueryResult<Vec<Project>> = projects::table.load::<Project>(conn);
    return response;
}


pub fn delete_project(id: String, conn: &PgConnection) -> Result<bool, Box<dyn Error>> {
    //! deletes the [Project] with given id, if successful returns true. If failed it returns a false.
    let response = diesel::delete(projects::table.filter(projects::id.eq(id))).execute(conn)?;
    if response == 1 {
        Ok(true)
    }else{
    Err(Box::new(DaoError::Failed))
}}