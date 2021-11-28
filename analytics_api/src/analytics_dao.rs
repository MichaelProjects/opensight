use super::schema::analytics;
use crate::analytics::AnalyticEntry;
use crate::dao::Dao;
use crate::schema::analytics::columns::{last_session, session_id};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use log::debug;

pub struct AnalyticsDao {}
impl Dao<AnalyticEntry, AnalyticEntry> for AnalyticsDao {
    fn new() -> Self {
        AnalyticsDao {}
    }

    fn insert_entry(&self, data: AnalyticEntry, conn: &mut PgConnection) -> bool {
        let successful = true;
        let response: QueryResult<AnalyticEntry> = diesel::insert_into(analytics::table)
            .values(&data)
            .get_result(conn);
        debug!("{:?}", response);
        successful
    }

    fn delete_entry(&self, id: &str, conn: &mut PgConnection) {
        let result = diesel::delete(analytics::table.filter(session_id.eq(id)))
            .get_result::<AnalyticEntry>(conn)
            .expect("could not find entry");
    }

    /// [update_entry] function in analytics dao is used to update the [session_length]
    /// using the [session_id] and returns the result of that operation.
    fn update_entry(&self, id: &str, update: i32, conn: &mut PgConnection) {
        let _result = diesel::update(analytics::table.filter(session_id.eq(id)))
            .set(last_session.eq(update))
            .get_result::<AnalyticEntry>(conn)
            .expect("");
    }

    fn get_entry(&self, _id: &str, _conn: &mut PgConnection) -> AnalyticEntry {
        todo!()
    }

    fn get_all(&self, conn: &mut PgConnection) -> Vec<AnalyticEntry> {
        let response: Vec<AnalyticEntry> = analytics::table
            .load::<AnalyticEntry>(conn)
            .expect("Entrys");
        return response;
    }
}
