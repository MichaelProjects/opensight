use postgres::{Client};
use crate::dao::Dao;
use crate::application::{Application, ApplicationType};

pub struct ApplicationDao{
    pub(crate) value: Vec<Application>

}
impl Dao<Vec<Application>> for ApplicationDao{
    fn new() -> Self {
        ApplicationDao{value: vec![]
        }
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

    fn get_entry(&self, conn: &mut Client) -> Vec<Application>{
        let query = "SELECT application_id, application_name, created_time, token, os from application;";

        let mut applications = vec![];

        let data = conn.query(query, &[]).unwrap();
        for row in data.iter(){
            applications.push(Application{
                name: row.get(1),
                os: ApplicationType::from_str(row.get(4)),
                uuid: row.get(0),
                added: row.get(2),
                token: row.get(3)
            })
            
        }
        applications
    }
}