use record::record::{NewRecord, Record};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod schema;
// pub mod record;

pub mod record;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn get_all_records() {
    use self::schema::record::dsl::*;

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
    use self::schema::record::dsl::{record, amount};

    let connection = &mut establish_connection();
    diesel::update(record.find(id))
        .set(amount.eq(new_amount))
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Could not update record")
}

// pub fn do_work(){
//     let record = Record::new(25.4);
//     let record2 = Record::new(11.2);

//     let mut all_records: Vec<Record> = Vec::new();
//     all_records.push(record);
//     all_records.push(record2);

//     println!("All records: {:?}", &all_records);
//     println!("Single record:");
//     println!("{}", all_records.get(0).unwrap());

//     let sum: f64 = all_records.iter().sum();
//     println!("Sum of all: {:.2}", sum);
// }