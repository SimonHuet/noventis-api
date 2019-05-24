use diesel::insert_into;
use diesel::select;
#[cfg(test)]
use diesel::debug_query;
#[cfg(test)]
use diesel::pg::Pg;

#[test]
fn examine_sql_from_insert_default_values() {
    use schema::doctors::dsl::*;

    let query = insert_into(doctors).default_values();
    let sql = "INSERT INTO \"doctors\" DEFAULT VALUES -- binds: []";
    assert_eq!(sql, debug_query::<Pg, _>(&query).to_string());
}

/*fn examine_sql_get_doctors(){
    use schema::doctors::dsl::*;
    
    let query = select(doctors).VALUES(1);
    let sql = "SELECT * FROM \"doctors\" WHERE id=1";
    assert_eq!(sql, debug_query::<Pg, _>(&query).to_string());

}*/