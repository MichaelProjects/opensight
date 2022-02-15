table! {
    analytics (session_id, application_id) {
        session_id -> Text,
        application_id -> Text,
        creation_time -> Timestamp,
        os -> Text,
        device_size -> Text,
        new_user -> Bool,
        country -> Text,
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
    opensight_user (userid, group_id) {
        userid -> Text,
        group_id -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        creation_time -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Text,
        application_id -> Text,
        length -> Int4,
        start_time -> Timestamp,
    }
}

table! {
    user_group (group_id) {
        group_id -> Text,
        name -> Text,
    }
}

joinable!(opensight_user -> user_group (group_id));

allow_tables_to_appear_in_same_query!(
    analytics,
    applications,
    opensight_user,
    sessions,
    user_group,
);
