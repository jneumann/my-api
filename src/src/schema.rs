table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    todo (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        checked -> Nullable<Bool>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    todo,
);
