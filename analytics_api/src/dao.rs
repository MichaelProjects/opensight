use rocket_sync_db_pools::diesel::PgConnection;

pub trait Dao<T, Y> {
    fn new() -> Self;
    fn insert_entry(&self, data: Y, conn: &mut PgConnection) -> bool;
    fn delete_entry(&self, id: &str, conn: &mut PgConnection);
    fn update_entry(&self, id: &str, update: i32, conn: &mut PgConnection);
    fn get_entry(&self, id: &str, conn: &mut PgConnection) -> T;
    fn get_all(&self, conn: &mut PgConnection) -> Vec<Y>;
}
