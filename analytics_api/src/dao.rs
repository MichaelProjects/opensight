use postgres::Client;

pub trait Dao{
    fn new() -> Self;
    fn insert_entry(&self);
    fn delete_entry(&self);
    fn update_entry(&self, conn: &mut Client);
    fn get_entry(&self, conn: &mut Client);
}