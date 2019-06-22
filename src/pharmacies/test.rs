use diesel::insert_into;
#[cfg(test)]
use diesel::debug_query;
#[cfg(test)]
use diesel::pg::Pg;

#[test]
fn examine_sql_from_insert_default_values() {
    use schema::pharmacies::dsl::*;

    let query = insert_into(pharmacies).default_values();
    let sql = "INSERT INTO \"pharmacies\" DEFAULT VALUES -- binds: []";
    assert_eq!(sql, debug_query::<Pg, _>(&query).to_string());
}