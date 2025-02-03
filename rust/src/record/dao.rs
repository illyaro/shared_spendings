use crate::record::record::{NewRecord, Record};
use diesel::prelude::*;
use crate::database_connection::databse_connetion::establish_connection;

pub fn get_all_records() {
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

pub fn add_record(amount: f64) -> Record{
    use crate::schema::record;
    let connection = &mut establish_connection();
    let new_record = NewRecord::new(amount);
    
    diesel::insert_into(record::table)
        .values(&new_record)
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Error adding new record to database.")
}

pub fn edit_record_amount(id: i64, new_amount: f64) -> Record {
    use crate::schema::record::dsl::{record, amount};

    let connection = &mut establish_connection();
    diesel::update(record.find(id))
        .set(amount.eq(new_amount))
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Could not update record")
}