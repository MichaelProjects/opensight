table! {
    analytics (session_id, application_id) {
        session_id -> Text,
        application_id -> Text,
        creation_time -> Timestamp,
        os -> Text,
        device_size -> Text,
        new_user -> Bool,
        country -> Text,
        session_length -> Int4,
        device_type -> Text,
        version -> Text,
    }
}

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
    sessions (id) {
        id -> Text,
        application_id -> Text,
        length -> Int4,
        is_first_login_today -> Bool,
        start_time -> Timestamp,
    }
}

table! {
    user_group (group_id) {
        group_id -> Text,
        name -> Text,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        email -> Varchar,
        salt -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        subscriptions -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    analytics,
    applications,
    sessions,
    user_group,
    users,
);
