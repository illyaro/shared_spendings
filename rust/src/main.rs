use shared_spendings::record::dao::*;

fn main() {
    get_all();
    add(String::from("123"), 14.12);
    get_all();
}
