use crate::dao::Dao;
use crate::analytics::{AnalyticEntry};
use diesel::{PgConnection, RunQueryDsl, QueryResult};
use super::schema::analytics;
use log::{debug};
pub struct AnalyticsDao{

}
impl Dao<AnalyticEntry, AnalyticEntry> for AnalyticsDao{
    fn new() -> Self {
        AnalyticsDao{}
    }

    fn insert_entry(&self, data: AnalyticEntry, conn: &mut PgConnection) -> bool {
        let successful = true;
        let response: QueryResult<AnalyticEntry> = diesel::insert_into(analytics::table)
            .values(&data)
            .get_result(conn);
        debug!("{:?}", response);
        successful
    }

    fn delete_entry(&self, _id: &str, _conn: &mut PgConnection) {
        todo!()
    }

    fn update_entry(&self, _id: &str, _conn: &mut PgConnection) {
        todo!()
    }

    fn get_entry(&self, _id: &str, _conn: &mut PgConnection) -> AnalyticEntry {
        todo!()
    }

    fn get_all(&self, conn: &mut PgConnection) -> Vec<AnalyticEntry> {
        let response: Vec<AnalyticEntry> = analytics::table.load::<AnalyticEntry>(conn).expect("Entrys");
        return response;
    }
}