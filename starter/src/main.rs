use financial_lib::db::*;
use financial_lib::record::{ExpenseType, MoneyType, Record};

use chrono::NaiveDate;

fn main() {
    let test_records = vec![
        Record {
            id: 21,
            money_type: MoneyType::EXPENSE,
            amount: 1000.0,
            expense: Some(ExpenseType::CAR),
            time: NaiveDate::from_ymd_opt(2026, 1, 24).unwrap(),
        },
        Record {
            id: 22,
            money_type: MoneyType::EXPENSE,
            amount: 600.0,
            expense: Some(ExpenseType::FREETIME),
            time: NaiveDate::from_ymd_opt(2026, 12, 25).unwrap(),
        },
        Record {
            id: 23,
            money_type: MoneyType::INCOME,
            amount: 2000.0,
            expense: None,
            time: NaiveDate::from_ymd_opt(2024, 11, 22).unwrap(),
        },
        Record {
            id: 24,
            money_type: MoneyType::EXPENSE,
            amount: 50.0,
            expense: Some(ExpenseType::CLOTH),
            time: NaiveDate::from_ymd_opt(2025, 9, 22).unwrap(),
        },
    ];

    for r in test_records {
        insert_record(&r); // vloží do databázy
    }
}
