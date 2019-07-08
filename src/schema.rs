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

table! {
    pharmacies_purchases_products (products_id, pharmacies_id) {
        quantity -> Int4,
        date -> Varchar,
        price -> Int4,
        pharmacies_id -> Int4,
        products_id -> Int4,
    }
}

table! {
    pharmacies_sales_products (products_id, pharmacies_id) {
        quantity -> Int4,
        date -> Varchar,
        price -> Int4,
        pharmacies_id -> Int4,
        products_id -> Int4,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(doctors -> pharmacies (pharmacy_id));
joinable!(doctors_formations -> doctors (doctor_id));
joinable!(doctors_formations -> formations (formation_id));
joinable!(pharmacies_purchases_products -> pharmacies (pharmacies_id));
joinable!(pharmacies_purchases_products -> products (products_id));
joinable!(pharmacies_sales_products -> pharmacies (pharmacies_id));
joinable!(pharmacies_sales_products -> products (products_id));

allow_tables_to_appear_in_same_query!(
    doctors,
    doctors_formations,
    formations,
    forms,
    pharmacies,
    pharmacies_purchases_products,
    pharmacies_sales_products,
    products,
);
