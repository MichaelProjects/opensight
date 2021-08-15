extern crate diesel;

use crate::dao::Dao;
use crate::application::{Application};
use diesel::{PgConnection, RunQueryDsl};
use super::schema::applications;

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

        true
    }

    fn delete_entry(&self, conn: &mut PgConnection) {
        todo!()
    }

    fn update_entry(&self, conn: &mut PgConnection) {
        todo!()
    }

    fn get_entry(&self, conn: &mut PgConnection) -> Vec<Application>{
        let result = vec![];
        result
    }
}