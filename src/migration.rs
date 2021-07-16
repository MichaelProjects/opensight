use postgres::{Client, NoTls, Error};


pub trait Migration{
    fn new(table_name: String, connection_str: String) -> Self;
    fn create(&mut self) -> bool;
    fn does_exist(&mut self, err: &Error) -> bool;
    fn delete(&mut self) -> bool;
    fn resolve(&mut self);
}

pub fn build_connection_st(host: String,dbname: String, user: String, password: String) -> String {
    format!("host={} dbname={} user={} password={}", host, dbname, user, password)
}


pub struct MigrateUserTable{
    pub done: bool,
    table_name: String,
    client: Client
}

impl Migration for MigrateUserTable{

    fn new(table_name: String, connection_str: String) -> Self{
        MigrateUserTable{
            done: false,
            table_name,
            client: match Client::connect(connection_str.as_str(), NoTls) {
                Ok(client) => client,
                Err(e) => panic!("Error while connecting to db: {}", e)
            }
        }
    }

    fn create(&mut self) -> bool {
        let query = format!("CREATE TABLE {} (userid text NOT NULL, username text NOT NULL, email text NOT NULL, password text NOT NULL, creation_date TIMESTAMP WITH TIME ZONE NOT NULL, power_level text NOT NULL, CONSTRAINT PK_user PRIMARY KEY ( userid ));", self.table_name);
        info!("create user table");
        let response = match &self.client.batch_execute(query.as_str()){
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

pub struct MigrateAnalyticsTable{
    pub done: bool,
    table_name: String,
    client: Client
}

impl Migration for MigrateAnalyticsTable{

    fn new(table_name: String, connection_str: String) -> Self{
        MigrateAnalyticsTable{
            done: false,
            table_name,
            client: match Client::connect(connection_str.as_str(), NoTls) {
                Ok(client) => client,
                Err(e) => panic!("Error while connecting to db: {}", e)
            }
        }
    }

    fn create(&mut self) -> bool {
        let query = format!("
        CREATE TABLE {}(tracking_id text NOT NULL, creation_time TIMESTAMP WITH TIME ZONE NOT NULL, os text NOT NULL, device_size text NOT NULL, session_length bigint NOT NULL, session_id text NOT NULL, CONSTRAINT PK_analytics PRIMARY KEY ( tracking_id ));", self.table_name);
        info!("Create Analytics Table");
        let response = match &self.client.batch_execute(query.as_str()){
            Ok(_response) => true,
            Err(err) => self.does_exist(err)
        };

        response
    }
    fn does_exist(&mut self, err: &Error) -> bool {
        let str = err.to_string();
        if str.contains("relation \"analytics\" already exists"){
            info!("Table does already exist");
            return true;
        }
        info!("Error: Analytics Table creation: {}", str);
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

