pub mod db;
pub mod models;
pub mod schema;

pub mod record {
    use std::{collections::HashMap, vec};

    use chrono::NaiveDate;

    #[derive(Debug,PartialEq, Clone, Copy)]
    pub enum MoneyType {
        INCOME,
        EXPENSE,
    }
    #[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
    pub enum ExpenseType {
        FUN,
        RESTAURANT,
        SHOPPING,
        INVESTMENT,
        FREETIME,
        HOME,
        CLOTH,
        CAR,
        TRAVEL,
        OTHER,
    }
    #[derive(Clone, Copy)]
    pub struct Record {
        pub id: i32,
        pub money_type: MoneyType,
        pub amount: f32,
        pub expense: Option<ExpenseType>,
        pub time: NaiveDate,
    }
    pub struct RecordManager {
        records: Vec<Record>,
    }
    impl RecordManager {
        pub fn new() -> RecordManager {
            RecordManager { records: vec![] }
        }
        pub fn add_record(&mut self, record: Record) {
            self.records.push(record);
        }
        pub fn get_record_id(&self, id: i32) -> Option<&Record> {
            let ret = self.records.iter().find(|r| r.id == id);
            ret
        }
        pub fn get_balance(&self) -> f32 {
            let mut ret = 0.00;
            for r in &self.records {
                ret += r.amount;
            }
            ret
        }
        pub fn get_expanses(&self) -> f32 {
            self.records
                .iter()
                .filter(|r| r.money_type == MoneyType::EXPENSE)
                .map(|r| r.amount)
                .sum()
        }
        pub fn get_income(&self) -> f32 {
            self.records
                .iter()
                .filter(|r| r.money_type == MoneyType::INCOME)
                .map(|r| r.amount)
                .sum()
        }
        pub fn records_between(&self, from: NaiveDate, to: NaiveDate) -> Vec<&Record> {
            let mut ret = Vec::new();

            for r in &self.records {
                if r.time <= to && r.time >= from {
                    ret.push(r);
                }
            }

            ret
        }

        pub fn expanse_by_category(&self) -> HashMap<ExpenseType, f32> {
            let mut ret = HashMap::new();
            let fun = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::FUN))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::FUN, fun);
            let restaurant = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::RESTAURANT))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::RESTAURANT, restaurant);
            let shopping = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::SHOPPING))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::SHOPPING, shopping);
            let investment = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::INVESTMENT))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::INVESTMENT, investment);
            let freetime = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::FREETIME))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::FREETIME, freetime);
            let home = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::HOME))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::HOME, home);
            let cloth = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::CLOTH))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::CLOTH, cloth);
            let travel = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::TRAVEL))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::TRAVEL, travel);
            let other = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::OTHER))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::OTHER, other);
            let car = self
                .records
                .iter()
                .filter(|r| r.expense == Some(ExpenseType::CAR))
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::CAR, car);

            ret
        }
        pub fn get_all(&self) -> Vec<Record> {
            let mut ret = Vec::new();
            for r in &self.records {
                let pusher = r.clone();
                ret.push(pusher);
            }
            ret
        }
    }
}
