use crate::record::record::{NewRecord, Record};
use diesel::prelude::*;
use crate::database_connection::databse_connetion::establish_connection;

pub fn get_all() {
    use crate::schema::record::dsl::*;

    let connection = &mut establish_connection();
    let result = record
        .limit(5)
        .select(Record::as_select())
        .load(connection)
        .expect("Error loading records");

    println!("Displaying {} Records", result.len());
    for rec in result {
        println!("{}", rec);
    }
}

pub fn get_one<'a>(id: i64) -> Option<Record<'a>> {
    use crate::schema::record::dsl::record;
    let connection = &mut establish_connection();

    let rec = record
        .find(id)
        .select(Record::as_select())
        .first(connection)
        .optional();

    match rec {
        Ok(Some(rec)) => Some(rec),
        Ok(None) => None,
        Err(_) => {
            println!("Could not fetch record");
            None
        }
    }
}

pub fn add<'a>(user_id: &'a str, amount: f64) -> Record<'a>{
    use crate::schema::record;
    let connection = &mut establish_connection();
    let new_record = NewRecord::new(&user_id, amount);
    
    diesel::insert_into(record::table)
        .values(&new_record)
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Error adding new record to database.")
}

pub fn edit_amount<'a>(id: i64, new_amount: f64) -> Record<'a> {
    use crate::schema::record::dsl::{record, amount};

    let connection = &mut establish_connection();
    diesel::update(record.find(id))
        .set(amount.eq(new_amount))
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Could not update record")
}

pub fn delete_one(record_id: i64) -> usize {
    use crate::schema::record::dsl::*;
    let connection = &mut establish_connection();

    diesel::delete(record.filter(id.eq(record_id)))
        .execute(connection)
        .expect("Error deleting records")
}