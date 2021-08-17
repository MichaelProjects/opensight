use crate::dao::Dao;
use crate::analytics::{AnalyticEntry, InsertableAnalytic};
use diesel::{PgConnection, QueryResult};
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
            .values(&InsertableAnalytic::from_analytics_entry(data))
            .get_results(conn);
    }

    fn delete_entry(&self, conn: &mut PgConnection) {
        todo!()
    }

    fn update_entry(&self, conn: &mut PgConnection) {
        todo!()
    }

    fn get_entry(&self, conn: &mut PgConnection) -> AnalyticEntry {
        todo!()
    }
}