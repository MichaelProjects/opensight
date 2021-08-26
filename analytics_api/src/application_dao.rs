use crate::dao::Dao;
use crate::application::*;
use diesel::{PgConnection, RunQueryDsl, QueryResult};
use super::schema::applications;
use log::{debug};
use diesel::prelude::*;


pub struct ApplicationDao {
    pub value: Vec<Application>

}
impl Dao<Vec<Application>, Application> for ApplicationDao{
    fn new() -> Self {
        ApplicationDao{value: vec![]
        }
    }

    fn insert_entry(&self, data: Application, conn: &mut PgConnection) -> bool {
        let mut successful = true;
        let response: QueryResult<Application> = diesel::insert_into(applications::table)
            .values(&data)
            .get_result(conn);
        true
    }

    fn delete_entry(&self, id: &str, conn: &mut PgConnection) {
        let mut successful = true;
        let result = diesel::delete(applications::table.find(id))
        .execute(conn);

    }

    fn update_entry(&self, id: &str, conn: &mut PgConnection) {
        todo!()
    }

    fn get_entry(&self, id: &str, conn: &mut PgConnection) -> Vec<Application>{
        vec![]
    }

    fn get_all(&self, conn: &mut PgConnection) -> Vec<Application> {
        let response: QueryResult<Vec<Application>> = applications::table.load::<Application>(&*conn);
        for x in response.iter(){
        }
        response.unwrap()
    }
}

