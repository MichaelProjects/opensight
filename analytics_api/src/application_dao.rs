use postgres::{Client};
use crate::dao::Dao;
use crate::application::{Application, ApplicationType};
use chrono::{DateTime, Utc};

pub struct ApplicationDao{
    pub(crate) value: Vec<Application>

}
impl Dao<Vec<Application>, Application> for ApplicationDao{
    fn new() -> Self {
        ApplicationDao{value: vec![]
        }
    }

    fn insert_entry(&self, data: Application, conn: &mut Client) -> bool {
        let mut successful = true;
        let query = "INSERT INTO applications (application_id, application_name, created_time, token, os) values ($1,$2,$3,$4,$5,$6)".to_string();
        let response = match conn.execute(query.as_str(),&[&data.uuid, &data.name, &data.added, &data.token, &data.os.as_str()]){
            Ok(response) => response,
            Err(err) => panic!("Error while inserting: {}", err)
        };
        println!("Rows Affected: {}", response);
        if response == 0{
            successful = false;
        }
        successful
    }

    fn delete_entry(&self, conn: &mut Client) {
        todo!()
    }

    fn update_entry(&self, conn: &mut Client) {
        todo!()
    }

    fn get_entry(&self, conn: &mut Client) -> Vec<Application>{
        let query = "SELECT application_id, application_name, created_time, token, os from applications;";

        let mut applications = vec![];

        let data = conn.query(query, &[]).unwrap();

        for row in data.iter(){
            let timestamp: DateTime<Utc> = row.get(2);
            println!("{:?}", timestamp);
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