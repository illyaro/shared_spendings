use record::record::Record;
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

pub fn try_db() {
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

pub fn do_work(){
    let record = Record::new(25.4);
    let record2 = Record::new(11.2);

    let mut all_records: Vec<Record> = Vec::new();
    all_records.push(record);
    all_records.push(record2);

    println!("All records: {:?}", &all_records);
    println!("Single record:");
    println!("{}", all_records.get(0).unwrap());

    let sum: f64 = all_records.iter().sum();
    println!("Sum of all: {:.2}", sum);
}