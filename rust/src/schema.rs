// @generated automatically by Diesel CLI.

diesel::table! {
    record (id) {
        id -> Int8,
        #[max_length = 21]
        user_id -> Varchar,
        amount -> Float8,
        dt -> Timestamp,
    }
}

diesel::table! {
    username (id) {
        #[max_length = 21]
        id -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 150]
        picture -> Varchar,
        #[max_length = 100]
        user_name -> Varchar,
    }
}

diesel::joinable!(record -> username (user_id));

diesel::allow_tables_to_appear_in_same_query!(record, username,);
