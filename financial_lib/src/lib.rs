//! Finančný pomocník – knižnica
//!
//! Tento crate poskytuje dátový model a databázovú logiku
//! pre aplikáciu na správu osobných financií.
//!
//! Obsahuje:
//! - databázovú vrstvu (`db`)
//! - dátové modely (`models`)
//! - definície záznamov a ich správu (`record`)
//! - manažéra na správu (`RecordManager`)
 
pub mod db;
pub mod models;
pub mod schema;
/// Modul obsahujúci dátové štruktúry pre finančné záznamy
/// a ich správu v pamäti.
pub mod record {
    use std::{collections::BTreeMap, vec};

    use chrono::NaiveDate;
    /// Typ finančnej operácie.
    ///
    /// Určuje, či ide o príjem alebo výdavok.
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum MoneyType {
        INCOME,
        EXPENSE,
    }
     /// Kategória výdavku.
    ///
    /// Používa sa iba pri výdavkoch.
    #[derive(Debug, PartialEq, Hash, Eq, Clone, Copy, PartialOrd, Ord)]
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
/// Prevod typu výdavku na textovú reprezentáciu.
    impl From<ExpenseType> for &str {
        fn from(value: ExpenseType) -> Self {
            let ret = match value {
                ExpenseType::FUN => "Fun",
                ExpenseType::RESTAURANT => "Restaurant",
                ExpenseType::SHOPPING => "Shopping",
                ExpenseType::INVESTMENT => "Investment",
                ExpenseType::FREETIME => "Freetime",
                ExpenseType::HOME => "Home",
                ExpenseType::CLOTH => "Cloth",
                ExpenseType::CAR => "Car",
                ExpenseType::TRAVEL => "Travel",
                ExpenseType::OTHER => "Other",
            };
            ret
        }
    }
/// Reprezentuje jeden finančný záznam.
    #[derive(Debug, Clone, Copy)]
    pub struct Record {
        pub id: i32,
        pub money_type: MoneyType,
        pub amount: f32,
        pub expense: Option<ExpenseType>,
        pub time: NaiveDate,
    }

    impl Record {
        /// Vytvorí nový finančný záznam.
///
/// # Arguments
/// * `id` – ID záznamu
/// * `mon_type` – typ peňazí (príjem/výdavok)
/// * `amount` – suma
/// * `expense` – kategória výdavku (ak ide o výdavok)
/// * `time` – dátum záznamu
        pub fn new(
           
            id: i32,
              
            mon_type: MoneyType,
             
            amount: f32,
           
            expense: Option<ExpenseType>,
               
            time: NaiveDate,
        ) -> Record {
            let ret = Record {
                id: id,
                money_type: mon_type,
                amount: amount,
                expense: expense,
                time: time,
            };
            ret
        }
        /// Vráti formátovaný textový zápis záznamu.
        pub fn format_record(&self) -> String {
            let mon_type = match self.money_type {
                MoneyType::INCOME => "+",
                MoneyType::EXPENSE => "-",
            };
            let expenseType = match self.expense {
                Some(e) => match e {
                    ExpenseType::FUN => "Fun",
                    ExpenseType::RESTAURANT => "Restaurant",
                    ExpenseType::SHOPPING => "Shopping",
                    ExpenseType::INVESTMENT => "Investment",
                    ExpenseType::FREETIME => "Freetime",
                    ExpenseType::HOME => "Home",
                    ExpenseType::CLOTH => "Cloth",
                    ExpenseType::CAR => "Car",
                    ExpenseType::TRAVEL => "Travel",
                    ExpenseType::OTHER => "Other",
                },
                None => "-",
            };
            let timeFormat = self.time.format("%d.%m.%Y").to_string();
            format! {"{:>3}  {:>1} {:>8.2}  {:<12}  {:<10}",self.id, mon_type, self.amount,expenseType,timeFormat}
        }
    }
  /// Správca finančných záznamov.
    #[derive(Debug)]
    pub struct RecordManager {
        records: Vec<Record>,
    }
    impl RecordManager {
        /// Vytvorí nový prázdny `RecordManager`.
        pub fn new() -> RecordManager {
            RecordManager { records: vec![] }
        }
///vráti naformátované všetky záznamy
        pub fn format_all(&self) -> Vec<String> {
            let ret = self.get_all().iter().map(|r| r.format_record()).collect();
            ret
        }
 /// Pridá nový záznam.
        pub fn add_record(&mut self, record: Record) {
            self.records.push(record);
        }
        /// Vráti záznam podľa ID.
        pub fn get_record_id(&self, id: i32) -> Option<&Record> {
            let ret = self.records.iter().find(|r| r.id == id);
            ret
        }
 /// Vráti celkový zostatok.
        pub fn get_balance(&self) -> f32 {
            let mut ret = 0.00;
            for r in &self.records {
                ret += r.amount;
            }
            ret
        }
     /// Vráti súčet výdavkov.
        pub fn get_expanses(&self) -> f32 {
            self.records
                .iter()
                .filter(|r| r.money_type == MoneyType::EXPENSE)
                .map(|r| r.amount)
                .sum()
        }
/// Vráti súčet príjmov.
        pub fn get_income(&self) -> f32 {
            self.records
                .iter()
                .filter(|r| r.money_type == MoneyType::INCOME)
                .map(|r| r.amount)
                .sum()
        }
/// Vráti záznamy medzi dvoma dátumami.
        pub fn records_between(&self, from: NaiveDate, to: NaiveDate) -> Vec<&Record> {
            let mut ret = Vec::new();

            for r in &self.records {
                if r.time <= to && r.time >= from {
                    ret.push(r);
                }
            }

            ret
        }
  /// Vráti súčet výdavkov rozdelený podľa kategórií.
        pub fn categories_to_hash(&self) -> BTreeMap<ExpenseType, f32> {
            let mut ret = BTreeMap::new();
            let fun = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::FUN) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::FUN, fun);

            let restaurant = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::RESTAURANT) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::RESTAURANT, restaurant);
            let shopping = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::SHOPPING) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::SHOPPING, shopping);
            let investment = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::INVESTMENT) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::INVESTMENT, investment);
            let freetime = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::FREETIME) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::FREETIME, freetime);
            let home = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::HOME) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::HOME, home);
            let cloth = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::CLOTH) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::CLOTH, cloth);
            let travel = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::TRAVEL) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::TRAVEL, travel);
            let other = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::OTHER) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::OTHER, other);
            let car = self
                .records
                .iter()
                .filter(|r| {
                    r.expense == Some(ExpenseType::CAR) && r.money_type == MoneyType::EXPENSE
                })
                .map(|r| r.amount)
                .sum();
            ret.insert(ExpenseType::CAR, car);

            ret
        }
          /// Vráti všetky záznamy ako nový vektor.
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
