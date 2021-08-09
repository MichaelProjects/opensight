table! {
    analytics (tracking_id, application_id) {
        tracking_id -> Text,
        application_id -> Text,
        creation_time -> Timestamp,
        os -> Text,
        device_size -> Text,
        session_length -> Int4,
        session_id -> Text,
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
