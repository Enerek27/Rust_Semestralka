//! Databázová vrstva aplikácie
//!
//! Tento modul zabezpečuje:
//! - pripojenie k SQLite databáze
//! - vkladanie, mazanie a aktualizáciu záznamov
//! - načítanie záznamov do pamäte
//!
//! Používa knižnicu **Diesel** a databázu **SQLite**.
//!
//! Databázová cesta sa načítava z premennej prostredia
//! `DATABASE_URL`.

use crate::schema;
use crate::schema::records::dsl::{id, records};
use crate::schema::records::{amount, expense, money_type, time};
use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use crate::models::dbRecord;
use crate::record::{Record, RecordManager};



/// Vytvorí a vráti spojenie so SQLite databázou.
///
/// Cesta k databáze sa načítava z premennej prostredia
/// `DATABASE_URL`.
///
/// # Panics
/// Ak premenná `DATABASE_URL` nie je nastavená alebo
/// ak sa nepodarí pripojiť k databáze.
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Vloží nový záznam do databázy.
///
/// # Argumenty
/// * `record` – záznam, ktorý sa má uložiť
///
/// # Panics
/// Ak sa nepodarí vykonať SQL INSERT.
pub fn insert_record(record: &Record) {
    let conn = &mut establish_connection();
    let insert_record: dbRecord = record.into();

    diesel::insert_into(schema::records::table)
        .values(insert_record)
        .execute(conn)
        .expect("Error inserting into db");
}

/// Načíta všetky záznamy z databázy.
///
/// Záznamy sú prevedené na typ [`Record`] a uložené
/// do [`RecordManager`].
///
/// # Returns
/// `RecordManager` obsahujúci všetky záznamy.
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
/// Aktualizuje existujúci záznam v databáze.
///
/// Záznam je identifikovaný pomocou jeho `id`.
///
/// # Argumenty
/// * `record` – záznam s novými hodnotami
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
/// Odstráni záznam z databázy.
///
/// # Argumenty
/// * `record` – záznam, ktorý sa má odstrániť
pub fn delete_record(record: Record) {
    let conn = &mut establish_connection();
    diesel::delete(records.filter(id.eq(record.id)))
        .execute(conn)
        .expect("Error deleting record from db");
}
/// Prečísluje všetky záznamy v databáze.
///
/// Záznamy sú:
/// 1. Načítané do pamäte
/// 2. Vymazané z databázy
/// 3. Znovu vložené s novým ID od 1
///
///  Používa sa pri mazaniach na zachovanie
/// postupného číslovania.
pub fn renumber_records_db() {
    let old = load_records();

    for r in old.get_all() {
        delete_record(r);
    }

    let old_records = old.get_all();
    let mut indexer = 1;
    for mut r in old_records {
        r.id = indexer;
        indexer += 1;
        insert_record(&r);
    }
}
/// Vráti ďalšie dostupné ID záznamu.
///
/// ID je určené na základe počtu záznamov
/// uložených v databáze.
///
/// # Returns
/// Nové ID záznamu.
pub fn get_next_id() -> i32 {
    let ret = load_records().get_all().len() + 1;
    ret as i32
}
