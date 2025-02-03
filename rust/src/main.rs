use chrono;
use shared_spendings::record::dao as record;
fn main() {
    let user_id = String::from("105273289810024784473");
    let dt_from =
        chrono::NaiveDateTime::parse_from_str("2025-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let dt_to =
        chrono::NaiveDateTime::parse_from_str("2025-01-31T23:59:59", "%Y-%m-%dT%H:%M:%S").unwrap();
    let records = record::get_all_of_user_and_date_interval(user_id, dt_from, dt_to);
    println!("total records: {}", records.len());
    for r in &records {
        println!("{}", r);
    }
    let total: f64 = records.iter().sum();
    println!("Total sum: {:.2}", total);
}
