use financial_lib::db::*;
use financial_lib::record::{Record, RecordManager, MoneyType, ExpenseType};
use chrono::NaiveDate;

fn main() {
    // 1️⃣ Vytvorenie testového záznamu
    let record = Record {
        id: 0, // id sa v DB nastaví automaticky
        money_type: MoneyType::INCOME,
        amount: 250.0,
        expense: None,
        time: NaiveDate::from_ymd_opt(2025, 12, 21).unwrap(),
    };

    // 2️⃣ Vloženie záznamu
    insert_record(&record);
    println!("✅ Record inserted!");

    // 3️⃣ Načítanie všetkých záznamov
    let  records: RecordManager = load_records();
    println!("📄 Loaded records");

    // 4️⃣ Aktualizácia záznamu (upravená suma)
    let vsetky = records.get_all();
    for r in vsetky.iter() {
        println!(
            "- ID: {}, Type: {:?}, Amount: {}, Expense: {:?}, Time: {}",
            r.id, r.money_type, r.amount, r.expense, r.time
        );
    }
    let  last = vsetky.iter().last().expect("Chyna poslednej");
    
    let mut change = last.clone();
    change.amount = 100.0;
    update_record(&change);
    println!("Record updated");

    // 5️⃣ Načítanie po aktualizácii
    let records_after_update = load_records();
    println!("📄 Records after update:");
    for r in records_after_update.get_all().iter() {
        println!(
            "- ID: {}, Type: {:?}, Amount: {}, Expense: {:?}, Time: {}",
            r.id, r.money_type, r.amount, r.expense, r.time
        );
    }

    // 6️⃣ Odstránenie prvého záznamu
    
    delete_record(change);
    println!("record removed");

    // 7️⃣ Načítanie po vymazaní
    let final_records = load_records();
    println!("📄 Records after delete: ");
}
