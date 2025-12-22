use financial_lib::db::*;
use financial_lib::record::{ExpenseType, MoneyType, Record};

use chrono::NaiveDate;

fn main() {
    let test_records = vec![
        Record {
            id: 13,
            money_type: MoneyType::EXPENSE,
            amount: 500.0,
            expense: None,
            time: NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
        },
        Record {
            id: 14,
            money_type: MoneyType::EXPENSE,
            amount: 150.0,
            expense: Some(ExpenseType::SHOPPING),
            time: NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
        },
        Record {
            id: 15,
            money_type: MoneyType::INCOME,
            amount: 200.0,
            expense: None,
            time: NaiveDate::from_ymd_opt(2025, 11, 22).unwrap(),
        },
        Record {
            id: 16,
            money_type: MoneyType::EXPENSE,
            amount: 50.0,
            expense: Some(ExpenseType::FUN),
            time: NaiveDate::from_ymd_opt(2025, 10, 22).unwrap(),
        },
    ];

    for r in test_records {
        insert_record(&r); // vloží do databázy
    }
}
