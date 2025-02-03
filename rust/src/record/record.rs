use std::{fmt::Display, iter::Sum};
use diesel::prelude::*;

#[derive(Debug, Default)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::record)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Record{
    id: i64,
    amount: f64,
}

impl Record {
    pub fn new(amount: f64) -> Self {
        let id = 0i64;
        Self {
            id,
            amount,
        }
    }

    pub fn new_clone(record: &Record) -> Self{
        Self {
            id: 0,
            amount: record.amount.clone(),
        }
    }
    
    pub fn edit(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn get_amount(&self) -> f64{
        self.amount
    }

}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {:10}, Amount: {}", self.id, self.amount)
    }
}

impl<'a> Sum<&'a Record> for f64 {
    fn sum<I: Iterator<Item = &'a Record>>(iter: I) -> f64 {
        iter.fold(0f64, |acc, record| acc + record.amount)
    }
}

// impl Add for Record {
//     type Output = Record;

//     fn add(self, rhs: Self) -> Self::Output {
//         Record{
//             amount: self.amount + rhs.amount
//         }
//     }
// }