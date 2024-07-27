// @generated automatically by Diesel CLI.

diesel::table! {
    choices (id) {
        id -> Nullable<Integer>,
        question_id -> Integer,
        choice_text -> Text,
        votes -> Nullable<Integer>,
    }
}

diesel::table! {
    questions (id) {
        id -> Nullable<Integer>,
        question_text -> Text,
        pub_date -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    choices,
    questions,
);
