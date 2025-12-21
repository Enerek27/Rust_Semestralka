use chrono::NaiveDate;
use diesel::prelude::*;

use crate::record::Record;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::records)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct dbRecord {
    pub id: Option<i32>,
    pub money_type: String,
    pub amount: f32,
    pub expense: Option<String>,
    pub time: String,
}

impl From<&Record> for dbRecord {
    fn from(value: &Record) -> Self {
        dbRecord {
            id: Some(value.id),
            money_type: match value.money_type {
                crate::record::MoneyType::INCOME => "INCOME".to_string(),
                crate::record::MoneyType::EXPENSE => "EXPENSE".to_string(),
            },
            amount: value.amount,
            expense: match value.expense {
                Some(e) => match e {
                    crate::record::ExpenseType::FUN => Some("FUN".to_string()),
                    crate::record::ExpenseType::RESTAURANT => Some("RESTAURANT".to_string()),
                    crate::record::ExpenseType::SHOPPING => Some("SHOPPING".to_string()),
                    crate::record::ExpenseType::INVESTMENT => Some("INVESTMENT".to_string()),
                    crate::record::ExpenseType::FREETIME => Some("FREETIME".to_string()),
                    crate::record::ExpenseType::HOME => Some("HOME".to_string()),
                    crate::record::ExpenseType::CLOTH => Some("CLOTH".to_string()),
                    crate::record::ExpenseType::CAR => Some("CAR".to_string()),
                    crate::record::ExpenseType::TRAVEL => Some("TRAVEL".to_string()),
                    crate::record::ExpenseType::OTHER => Some("OTHER".to_string()),
                },
                None => Some("NONE".to_string()),
            },
            time: value.time.format("%d.%m.%Y").to_string(),
        }
    }
}
impl From<&dbRecord> for Record {
    fn from(value: &dbRecord) -> Self {
        Record {
            id: match value.id {
                Some(i) => i,
                None => panic!("Wrong value from database\n"),
            },
            money_type: match value.money_type.as_str() {
                "INCOME" => crate::record::MoneyType::INCOME,
                "EXPENSE" => crate::record::MoneyType::EXPENSE,
                _ => panic!("Error while parsing moneyType from db"),
            },
            amount: value.amount,
            expense: match &value.expense {
                Some(e) => match e.as_str() {
                    "FUN" => Some(crate::record::ExpenseType::FUN),
                    "RESTAURANT" => Some(crate::record::ExpenseType::RESTAURANT),
                    "SHOPPING" => Some(crate::record::ExpenseType::SHOPPING),
                    "INVESTMENT" => Some(crate::record::ExpenseType::INVESTMENT),
                    "FREETIME" => Some(crate::record::ExpenseType::FREETIME),
                    "HOME" => Some(crate::record::ExpenseType::HOME),
                    "CLOTH" => Some(crate::record::ExpenseType::CLOTH),
                    "CAR" => Some(crate::record::ExpenseType::CAR),
                    "TRAVEL" => Some(crate::record::ExpenseType::TRAVEL),
                    "OTHER" => Some(crate::record::ExpenseType::OTHER),
                    "NONE" => None,
                    _ => panic!("Error while parsing expense from db"),
                },
                None => None,
            },
            time: NaiveDate::parse_from_str(&value.time, "%d.%m.%Y")
                .expect("Error while parsing time from db"),
        }
    }
}
