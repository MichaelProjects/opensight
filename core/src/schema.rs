table! {
    applications (application_id) {
        application_id -> Text,
        application_name -> Text,
        package_name -> Text,
        creation_time -> Timestamp,
        token -> Text,
        os -> Text,
    }
}

table! {
    projects (id) {
        id -> Text,
        projects_name -> Text,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    user_group (group_id) {
        group_id -> Text,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        group_id -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        creation_time -> Timestamp,
        pepper -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    applications,
    projects,
    user_group,
    users,
);
