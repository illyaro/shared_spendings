
// pub mod record;
pub mod schema;
pub mod record;
pub mod user;
pub mod database_connection;

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