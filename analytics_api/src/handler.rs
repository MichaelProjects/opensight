/// As you can see, in this file are the non admin Rest-Endpoints.
/// These Endpoints are used to collect/recieve data from the clients using the Opensight SDK's.

use crate::analytics::{AnalyticEntry, AnalyticData, SessionUpdate};
use rocket::serde::json::Json;
use crate::{health};
use crate::db::{AnalyticsDB};
use rocket::http::Status;

use crate::application_dao::ApplicationDao;
use crate::dao::Dao;

#[get("/health")]
pub(crate) async fn get_health(_conn: AnalyticsDB) -> Json<health::Health>{
    Json(health::get_health_state())
}

#[post("/<application_id>/entry", data="<analytics>")]
pub(crate) async fn insert_entry(conn: AnalyticsDB, application_id: String, analytics: Json<AnalyticData>) -> Status{
    let mut found = false;
    let apps = conn.run(|c| ApplicationDao::new().get_all(c)).await;
    for x in apps.iter(){
        if x.application_id == application_id{
            found = true;
        }
    }
    if !found{
        return Status::NotFound
    }
    let analytic_entry = AnalyticEntry::new(analytics.into_inner(), application_id);
    let _result = conn.run(|c| analytic_entry.insert_entry(c)).await;
    Status::Accepted
}

#[patch("/<application_id>/entry/session", data="<session_update>")]
pub(crate) async fn update_session(conn: AnalyticsDB, application_id: String, session_update: Json<SessionUpdate>) -> Status{
    let mut found = false;
    let apps = conn.run(|c| ApplicationDao::new().get_all(c)).await;
    for x in apps.iter(){
        if x.application_id == application_id{
            found = true;
        }
    }
    if !found{
        return Status::NotFound
    }


    Status::Accepted
}