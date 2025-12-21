use crate::schema;
use crate::schema::records::dsl::records;
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
    let vec: Vec<dbRecord> = records.limit(10).load(conn).expect("Error loading from db");
    let normal = vec.iter().map(|r| r.into());
    for r in normal {
        Rm.add_record(r);
    }
    Rm
}
