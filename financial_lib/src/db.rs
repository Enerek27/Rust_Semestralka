use crate::schema;
use crate::schema::records::dsl::{id, records};
use crate::schema::records::{amount, expense, money_type, time};
use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use crate::models::dbRecord;
use crate::record::{Record, RecordManager};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_record(record: &Record) {
    let conn = &mut establish_connection();
    let insert_record: dbRecord = record.into();

    diesel::insert_into(schema::records::table)
        .values(insert_record)
        .execute(conn)
        .expect("Error inserting into db");
}

pub fn load_records() -> RecordManager {
    let conn = &mut establish_connection();
    let mut Rm = RecordManager::new();
    let vec: Vec<dbRecord> = records.load(conn).expect("Error loading from db");
    let normal = vec.iter().map(|r| r.into());
    for r in normal {
        Rm.add_record(r);
    }
    Rm
}

pub fn update_record(record: &Record) {
    let conn = &mut establish_connection();
    let update_record: dbRecord = record.into();

    diesel::update(records.filter(id.eq(record.id)))
        .set((
            money_type.eq(update_record.money_type),
            amount.eq(update_record.amount),
            expense.eq(update_record.expense),
            time.eq(update_record.time),
        ))
        .execute(conn)
        .expect("Error updating db");
}

pub fn delete_record(record: Record) {
    let conn = &mut establish_connection();
    diesel::delete(records.filter(id.eq(record.id)))
        .execute(conn)
        .expect("Error deleting record from db");
}
