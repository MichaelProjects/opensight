use postgres::{Client, NoTls, Row};
use crate::dao::Dao;

pub struct ApplicationDao{
}
impl Dao for ApplicationDao{
    fn new() -> Self {
        todo!()
    }

    fn insert_entry(&self) {
        todo!()
    }

    fn delete_entry(&self) {
        todo!()
    }

    fn update_entry(&self, conn: &mut Client) {
        todo!()
    }

    fn get_entry(&self, conn: &mut Client) -> Vec<Row> {
        let query = "SELECT application_id, token from application;";
        let response = match conn.query(query, &[]){
            Ok(value) => value,
            Err(err) => error!("An Error ouccured while querying for application details: {}",err)
        };
        response
    }
}