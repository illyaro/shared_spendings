use crate::model::database_connection::databse_connetion::establish_connection;
use crate::model::record::record::{NewRecord, Record};
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;

pub fn get_all() -> Vec<Record> {
    use crate::model::schema::record::dsl::*;

    let connection = &mut establish_connection();
    record
        .select(Record::as_select())
        .load(connection)
        .expect("Error loading records")
}

pub fn get_all_of_user_and_date_interval(
    usr_id: Option<String>,
    d_from: Option<NaiveDateTime>,
    d_to: Option<NaiveDateTime>,
) -> Vec<Record> {
    use crate::model::schema::record::dsl::*;

    let connection = &mut establish_connection();
    let mut query = record::table().into_boxed();

    if let Some(u_id) = usr_id {
        query = query.filter(user_id.eq(u_id));
    }
    if let Some(d_from) = d_from {
        query = query.filter(dt.ge(d_from));
    }
    if let Some(d_to) = d_to {
        query = query.filter(dt.le(d_to));
    } 
    query = query
        .order_by(dt);

    query
        .select(Record::as_select())
        .load::<Record>(connection)
        .expect("Error loading records")

    // record
    //     .filter(user_id.eq(usr_id))
    //     .filter(dt.ge(d_from))
    //     .filter(dt.le(d_to))
    //     .order_by(dt)
    //     .select(Record::as_select())
    //     .load(connection)
    //     .expect("Error loading records")
}

pub fn get_one(id: i64) -> Option<Record> {
    use crate::model::schema::record::dsl::record;
    let connection = &mut establish_connection();

    let rec = record
        .find(id)
        .select(Record::as_select())
        .first(connection)
        .optional();

    match rec {
        Ok(Some(rec)) => Some(rec),
        Ok(None) => None,
        Err(_) => {
            println!("Could not fetch record");
            None
        }
    }
}

pub fn add(new_record: NewRecord) -> Record {
    use crate::model::schema::record;
    let connection = &mut establish_connection();
    // let new_record = NewRecord::new(user_id, amount);

    // new_record.set_dt(dt);

    diesel::insert_into(record::table)
        .values(&new_record)
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Error adding new record to database.")
}

pub fn edit_amount(id: i64, new_amount: f64) -> Record {
    use crate::model::schema::record::dsl::{amount, record};

    let connection = &mut establish_connection();
    diesel::update(record.find(id))
        .set(amount.eq(new_amount))
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Could not update record")
}

pub fn edit_date(id: i64, new_date: NaiveDateTime) -> Record {
    use crate::model::schema::record::dsl::{dt, record};

    let connection = &mut establish_connection();
    diesel::update(record.find(id))
        .set(dt.eq(new_date))
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Could not update record")
}

pub fn edit(rec: Record) -> Record {
    use crate::model::schema::record::dsl::*;

    let connection = &mut establish_connection();
    diesel::update(record.find(rec.get_id()))
        .set(&rec)
        .returning(Record::as_returning())
        .get_result(connection)
        .expect("Could not update record")
}

pub fn delete_one(record_id: i64) -> usize {
    use crate::model::schema::record::dsl::*;
    let connection = &mut establish_connection();

    diesel::delete(record.filter(id.eq(record_id)))
        .execute(connection)
        .expect("Error deleting records")
}
