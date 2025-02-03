use shared_spendings::record::dao as record;

fn main() {
    let mut records = record::get_all();
    println!("total records: {}", records.len());
    for r in records{
        println!("{}", &r);
    }
    record::add(String::from("123"), 14.12);
    records = record::get_all();
    println!("total records: {}", records.len());
    for r in records{
        println!("{}", &r);
    }
}
