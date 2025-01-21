// @generated automatically by Diesel CLI.

diesel::table! {
    timeslot (id) {
        id -> Int4,
        trainer_id -> Int4,
        start -> Timestamp,
        duration -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        user_id -> Nullable<Int4>,
    }
}
