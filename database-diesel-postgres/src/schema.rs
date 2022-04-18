table! {
    events (id) {
        id -> Int4,
        timeline_id -> Int4,
        title -> Text,
        body -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        done -> Nullable<Bool>,
    }
}

table! {
    timelines (id) {
        id -> Int4,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        public -> Bool,
    }
}

table! {
    timelines_users (id) {
        id -> Int4,
        timeline_id -> Int4,
        user_id -> Int4,
        relation -> crate::db_types::UserRole,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
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
joinable!(timelines_users -> timelines (timeline_id));
joinable!(timelines_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    timelines,
    timelines_users,
    users,
);
