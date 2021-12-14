use crate::application::Application;
use diesel;
use super::schema::applications;

pub async fn insert_application(application: &Application, conn: &diesel::PgConnection) {
    let response = diesel::insert_into(application::tables)
    .values(application)
    .get_result(conn);
}