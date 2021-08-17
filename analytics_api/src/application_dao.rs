use diesel;
use crate::dao::Dao;
use crate::application::*;
use diesel::{PgConnection, RunQueryDsl, QueryResult};
use super::schema::applications;
use chrono::NaiveDateTime;

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
            .values(&InsertableApplication::from_application(data))
            .get_result(conn);
        for x in response.iter() {
            println!("{:?}", x);
        }
        true
    }

    fn delete_entry(&self, id: &str, conn: &mut PgConnection) {
        let mut successful = true;
        let result = diesel::delete(applications::table.find(id))
        .get_result(conn)
        
    }

    fn update_entry(&self, conn: &mut PgConnection) {
        todo!()
    }

    fn get_entry(&self, conn: &mut PgConnection) -> Vec<Application>{
        let result = vec![];
        result
    }
}

