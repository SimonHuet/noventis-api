table! {
    doctors (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        birthdate -> Varchar,
    }
}

table! {
    doctors_formations (id) {
        id -> Int4,
        doctor_id -> Int4,
        formation_id -> Int4,
    }
}

table! {
    formations (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    forms (id) {
        id -> Int4,
        created_at -> Varchar,
        form -> Json,
    }
}

joinable!(doctors_formations -> doctors (doctor_id));
joinable!(doctors_formations -> formations (formation_id));

allow_tables_to_appear_in_same_query!(
    doctors,
    doctors_formations,
    formations,
    forms,
);
