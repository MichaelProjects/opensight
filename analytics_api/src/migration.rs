use postgres::{Client, NoTls, Error};
use log::{info, warn};
use std::ops::Index;


pub fn check_db_tables(connection_str: String) {
    //! this function gets the connection string, and iterates over each query given in a vector,
    //! then it simply check if the table is present, if not it will try to create a table in the given database.
    let mut migrations_processes = vec![];
    let tables = vec!["analytics", "user_access", "applications"];
    let queries = ["CREATE TABLE user_access (userid text NOT NULL, username text NOT NULL, email text NOT NULL, password text NOT NULL, creation_date TIMESTAMP WITH TIME ZONE NOT NULL, power_level text NOT NULL, CONSTRAINT PK_user PRIMARY KEY ( userid ));", "CREATE TABLE Analytics (tracking_id text NOT NULL,application_id text NOT NULL,creation_time timestamp NOT NULL,os text NOT NULL,device_size text NOT NULL,session_length int NOT NULL,session_id text NOT NULL,CONSTRAINT PK_analytics PRIMARY KEY ( tracking_id, application_id ));", "CREATE TABLE applications ( application_id text NOT NULL, application_name text NOT NULL, created_time timestamp NOT NULL, token text NOT NULL, os text NOT NULL, CONSTRAINT PK_applications PRIMARY KEY ( application_id ) );"];
    let mut counter = 0;
    for name in tables.iter(){
        let index_value: &str = queries.index(counter);
        let a = Migrations::new(String::from(name.clone()), connection_str.clone(), String::from(index_value));
        migrations_processes.push(a);
        counter +=1;
    }
    for migration in migrations_processes.iter_mut(){
        migration.resolve();
    }
}

pub trait Migration{
    fn new(table_name: String, connection_str: String, creation_query: String) -> Self;
    fn create(&mut self) -> bool;
    fn does_exist(&mut self, err: &Error) -> bool;
    fn delete(&mut self) -> bool;
    fn resolve(&mut self);
}

pub fn build_connection_st(host: String,dbname: String, user: String, password: String) -> String {
    format!("host={} dbname={} user={} password={}", host, dbname, user, password)
}
pub struct Migrations{
    pub done: bool,
    table_name: String,
    client: Client,
    creation_query: String,
}
impl Migration for Migrations{
    fn new(table_name: String, connection_str: String, creation_query: String) -> Self{
        Migrations{
            done: false,
            table_name,
            creation_query,
            client: match Client::connect(connection_str.as_str(), NoTls) {
                Ok(client) => client,
                Err(e) => panic!("Error while connecting to db: {}", e)
            }
        }
    }

    fn create(&mut self) -> bool {
        info!("create user table");
        let response = match &self.client.batch_execute(self.creation_query.as_str()){
            Ok(_response) => true,
            Err(err) => self.does_exist(err)
        };

        response
    }
    fn does_exist(&mut self, err: &Error) -> bool {
        let str = err.to_string();
        println!("{}", str);
        if str.contains("relation \"user\" already exists"){
            info!("table does already exist");
            return true;
        }
        error!("Error: user table creation: {}", str);
        false
    }

    fn delete(&mut self) -> bool{
        false
    }
    fn resolve(&mut self){
        let created = self.create();
        if created{
            self.done = true;
        }else{
            self.done = false;
        }
    }
}

