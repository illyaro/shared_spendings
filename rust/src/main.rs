use shared_spendings::*;



fn main() {
    // let record = Record::new(2.2);
    // do_work();
    // let amount = 12.63f64;
    // let record = add_record(amount.clone());
    // println!("Was created a record with amount {}, here is record from db: {}", &amount, record);
    get_all_records();
    println!("Trying to edit a record");
    edit_record_amount(3, 13.48);
    println!("Success!");
    get_all_records();
    // println!("myrecord {}", record);
}
