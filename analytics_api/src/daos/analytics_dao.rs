use crate::analytics::AnalyticEntry;
use crate::dao::Dao;
use crate::db::AnalyticsDB;
use crate::schema::analytics;
use crate::schema::analytics::columns::{session_id};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use log::debug;
use chrono::{NaiveDateTime};

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

    fn get_entry(&self, _id: &str, _conn: &mut PgConnection) -> AnalyticEntry {
        todo!()
    }

    fn get_all(&self, conn: &mut PgConnection) -> Vec<AnalyticEntry> {
        let response: Vec<AnalyticEntry> = analytics::table
            .load::<AnalyticEntry>(conn)
            .expect("Entrys");
        return response;
    }

    fn update_entry(&self, id: &str, update: i32, conn: &mut PgConnection) {
        todo!()
    }
}

pub fn insert_entry(data: AnalyticEntry, conn: &mut PgConnection) -> QueryResult<AnalyticEntry> {
    let response: QueryResult<AnalyticEntry> = diesel::insert_into(analytics::table)
        .values(&data)
        .get_result(conn);
    return response
}

pub async fn get_all<'a>(app_id: String, conn: AnalyticsDB, limit: i64, start: NaiveDateTime, end: NaiveDateTime) -> Vec<AnalyticEntry>  {
    let response: QueryResult<Vec<AnalyticEntry>> = conn.run(move |c| analytics::table
        .filter(analytics::application_id.eq(app_id))
        .filter(analytics::creation_time.between(start, end))
        .limit(limit)
        .load::<AnalyticEntry>(c))
        .await;
    return response.expect("Entrys");
}

pub async fn get_timeframe_entry(app_id: String, conn: AnalyticsDB, start: NaiveDateTime, end: NaiveDateTime) -> QueryResult<Vec<AnalyticEntry>>{
    let response: QueryResult<Vec<AnalyticEntry>> = conn.run(move |c| 
        analytics::table
        .filter(analytics::application_id.eq(app_id))
        .filter(analytics::creation_time.between(start, end))
        .load::<AnalyticEntry>(c)).await;
        return response
}

pub async fn get_newuser_in_timeframe(app_id: String, conn: AnalyticsDB, start: NaiveDateTime, end: NaiveDateTime) -> QueryResult<Vec<AnalyticEntry>>{
    let response: QueryResult<Vec<AnalyticEntry>> = conn.run(move |c| 
        analytics::table
        .filter(analytics::application_id.eq(app_id))
        .filter(analytics::creation_time.between(start, end))
        .filter(analytics::new_user.eq(true))
        .load::<AnalyticEntry>(c)).await;
        return response
}