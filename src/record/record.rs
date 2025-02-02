use std::{fmt::Display, iter::Sum, ops::Add};

#[derive(Debug)]
pub struct Record{
    amount: f64,
}

impl Record {
    pub fn new(amount: f64) -> Self {
        Self {
            amount
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
        write!(f, "Amount: {}", self.amount)
    }
}

impl<'a> Sum<&'a Record> for f64 {
    fn sum<I: Iterator<Item = &'a Record>>(iter: I) -> f64 {
        iter.fold(0f64, |acc, record| acc + record.amount)
    }
}

impl Add for Record {
    type Output = Record;

    fn add(self, rhs: Self) -> Self::Output {
        Record{
            amount: self.amount + rhs.amount
        }
    }
}