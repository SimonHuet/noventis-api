table! {
    doctors (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        birthdate -> Timestamp,
    }
}

table! {
    formations (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    doctors,
    formations,
);
