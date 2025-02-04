use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::{fmt::Display, iter::Sum};

#[derive(Debug, Default, Queryable, Selectable)]
#[diesel(table_name = crate::model::schema::record)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Record {
    id: i64,
    user_id: String,
    amount: f64,
    dt: NaiveDateTime,
}

impl Record {
    pub fn new(user_id: String, amount: f64) -> Self {
        let id = 0i64;
        let dt = chrono::Local::now().naive_local();
        Self {
            id,
            user_id,
            amount,
            dt,
        }
    }

    pub fn new_clone(record: &Record) -> Self {
        Self {
            id: 0,
            user_id: record.user_id.clone(),
            amount: record.amount.clone(),
            dt: record.dt,
        }
    }

    pub fn edit(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ID: {:10}, {:23}, Amount: {}",
            self.id, self.dt, self.amount
        )
    }
}

impl<'a> Sum<&'a Record> for f64 {
    fn sum<I: Iterator<Item = &'a Record>>(iter: I) -> f64 {
        iter.fold(0f64, |acc, record| acc + record.amount)
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::model::schema::record)]
pub struct NewRecord {
    user_id: String,
    amount: f64,
    dt: NaiveDateTime,
}

impl NewRecord {
    pub fn new(user_id: String, amount: f64) -> Self {
        let dt = chrono::Local::now().naive_local();
        Self {
            user_id,
            amount,
            dt,
        }
    }
}
