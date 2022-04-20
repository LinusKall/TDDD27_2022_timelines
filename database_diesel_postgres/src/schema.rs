table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    events (id) {
        id -> Int4,
        timeline_id -> Int4,
        title -> Text,
        body -> Nullable<Text>,
        done -> Nullable<Bool>,
        start_time -> Nullable<Timestamp>,
        end_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    sub_events (id) {
        id -> Int4,
        event_id -> Int4,
        title -> Text,
        done -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    timelines (id) {
        id -> Int4,
        title -> Text,
        public -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    timelines_users (timeline_id, user_id) {
        timeline_id -> Int4,
        user_id -> Int4,
        relation -> Clearance_mapping,
        color -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(events -> timelines (timeline_id));
joinable!(sub_events -> events (event_id));
joinable!(timelines_users -> timelines (timeline_id));
joinable!(timelines_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    sub_events,
    timelines,
    timelines_users,
    users,
);
