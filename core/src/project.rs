

#[derive(Serialize, Clone, Debug, Hash, Queryable, Insertable)]
#[table_name = "projects"]
pub struct Projects {
    pub projects_id: String,
    pub project_name: String,
    pub created: NativeDateTime,
    pub updated: NativeDateTime
}