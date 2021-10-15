use crate::dao::Dao;
use crate::application::*;
use diesel::{PgConnection, RunQueryDsl, QueryResult};
use super::schema::applications;

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
        let _successful = true;
        let _response: QueryResult<Application> = diesel::insert_into(applications::table)
            .values(&data)
            .get_result(conn);
        true
    }

    fn delete_entry(&self, id: &str, conn: &mut PgConnection) {
        let _successful = true;
        let _result = diesel::delete(applications::table.find(id))
        .execute(conn);

    }

    fn update_entry(&self, _id: &str, _update: i32, _conn: &mut PgConnection) {
        todo!()
    }

    fn get_entry(&self, _id: &str, _conn: &mut PgConnection) -> Vec<Application>{
        vec![]
    }

    fn get_all(&self, conn: &mut PgConnection) -> Vec<Application> {
        let response: Vec<Application> = applications::table.load::<Application>(conn).expect("abc");
        response
    }
}

