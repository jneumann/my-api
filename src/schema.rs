table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Varchar,
        published -> Bool,
    }
}

table! {
    vehicles (id) {
        id -> Int4,
        make -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        vin -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
        starting_odometer -> Nullable<Numeric>,
        current_odometer -> Nullable<Numeric>,
        created_at -> Nullable<Timestamptz>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    vehicles,
);
