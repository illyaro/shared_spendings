use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, iter::Sum};

#[derive(Debug, Default, Queryable, AsChangeset, Selectable, Deserialize, Serialize)]
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

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn get_dt(&self) -> NaiveDateTime {
        self.dt
    }

    pub fn set_dt(&mut self, dt: NaiveDateTime) {
        self.dt = dt; // chrono::Local::now().naive_local();
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn set_user_id(&mut self, new_id: String) {
        self.user_id = new_id;
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

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::model::schema::record)]
pub struct NewRecord {
    user_id: String,
    amount: f64,
    // #[serde(skip_deserializing)]
    dt: NaiveDateTime,
}

impl NewRecord {
    pub fn new(user_id: String, amount: f64, dt: NaiveDateTime) -> Self {
        // let dt = Some(chrono::Local::now().naive_local());
        Self {
            user_id,
            amount,
            dt,
        }
    }

    pub fn set_dt(&mut self, dt: NaiveDateTime) {
        self.dt = dt; // chrono::Local::now().naive_local();
    }
}
