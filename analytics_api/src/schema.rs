table! {
    analytics (session_id, application_id) {
        session_id -> Text,
        application_id -> Text,
        creation_time -> Timestamp,
        os -> Text,
        device_size -> Text,
        new_user -> Bool,
        country -> Text,
        last_session -> Int4,
        device_type -> Text,
        version -> Text,
    }
}

table! {
    applications (application_id) {
        application_id -> Text,
        application_name -> Text,
        created_time -> Timestamp,
        token -> Text,
        os -> Text,
    }
}

table! {
    user_access (userid) {
        userid -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        creation_date -> Timestamptz,
        power_level -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    analytics,
    applications,
    user_access,
);
