table! {
    posts (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        published -> Nullable<Bool>,
    }
}

table! {
    vehicles (id) {
        id -> Int4,
        make -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        vin -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    vehicles,
);
