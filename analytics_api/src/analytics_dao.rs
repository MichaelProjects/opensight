use crate::dao::Dao;
use crate::analytics::{AnalyticEntry};
use diesel::{PgConnection, RunQueryDsl, QueryResult};
use super::schema::analytics;

pub struct AnalyticsDao{

}
impl Dao<AnalyticEntry, AnalyticEntry> for AnalyticsDao{
    fn new() -> Self {
        AnalyticsDao{}
    }

    fn insert_entry(&self, data: AnalyticEntry, conn: &mut PgConnection) -> bool {
        let mut successful = true;
        let response: QueryResult<AnalyticEntry> = diesel::insert_into(analytics::table)
            .values(&data)
            .get_result(conn);
        println!("{:?}", response);
        successful
    }

    fn delete_entry(&self, id: &str, conn: &mut PgConnection) {
        todo!()
    }

    fn update_entry(&self, id: &str, conn: &mut PgConnection) {
        todo!()
    }

    fn get_entry(&self, id: &str, conn: &mut PgConnection) -> AnalyticEntry {
        todo!()
    }

    fn get_all(&self, conn: &mut PgConnection) -> Vec<AnalyticEntry> {
        let result = vec![];
        result
    }
}