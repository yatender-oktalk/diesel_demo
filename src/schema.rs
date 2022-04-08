table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Nullable<Bool>,
        user_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        phone -> Varchar,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
