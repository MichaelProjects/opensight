use postgres::Client;

pub trait Dao<T, Y>{
    fn new() -> Self;
    fn insert_entry(&self, data: Y, conn: &mut Client) -> bool;
    fn delete_entry(&self, conn: &mut Client);
    fn update_entry(&self, conn: &mut Client);
    fn get_entry(&self, conn: &mut Client) -> T;
}