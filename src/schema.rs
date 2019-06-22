table! {
    doctors (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        birthdate -> Varchar,
        pharmacy_id -> Nullable<Int4>,
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

table! {
    pharmacies (id) {
        id -> Int4,
        street -> Varchar,
        street_number -> Int4,
        city -> Varchar,
        longitude -> Float8,
        latitude -> Float8,
    }
}

joinable!(doctors -> pharmacies (pharmacy_id));
joinable!(doctors_formations -> doctors (doctor_id));
joinable!(doctors_formations -> formations (formation_id));

allow_tables_to_appear_in_same_query!(
    doctors,
    doctors_formations,
    formations,
    forms,
    pharmacies,
);
