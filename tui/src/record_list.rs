//! Modul pre správu a výber záznamov (`Record`) v aplikácii.
//!
//! Obsahuje štruktúru [`RecordLister`], ktorá uchováva [`RecordManager`] a stav výberu (`ListState`),
//! a metódy na prechádzanie, pridávanie, aktualizovanie a mazanie záznamov.
use chrono::NaiveDate;

use financial_lib::{
    db::{
        delete_record, get_next_id, insert_record, load_records, renumber_records_db, update_record,
    },
    record::{ExpenseType, MoneyType, Record, RecordManager},
};
use ratatui::widgets::ListState;

/// Štruktúra na správu zoznamu záznamov s výberom.
#[derive(Debug)]
pub struct RecordLister {
    /// Správca záznamov.
    pub record_manager: RecordManager,
    /// Stav vybraného záznamu v UI.
    pub state: ListState,
}

impl RecordLister {
    /// Vytvorí nový [`RecordLister`] a načíta záznamy z databázy.
    pub fn new() -> Self {
        RecordLister {
            record_manager: load_records(),
            state: ListState::default(),
        }
    }
/// Posunie výber na ďalší záznam.
    pub fn select_next(&mut self) {
        if self.record_manager.get_all().len() == 0 {
            return;
        }

        let selected_now = match self.state.selected() {
            Some(i) => i,
            None => 0,
        };

        let lenght = self.record_manager.get_all().len();
        let mut select_next = 0;
        if selected_now == lenght - 1 {
            select_next = 0;
        } else {
            select_next = selected_now + 1;
        }

        self.state.select(Some(select_next));
    }

    /// Posunie výber na predchádzajúci záznam.
    pub fn select_previous(&mut self) {
        if self.record_manager.get_all().len() == 0 {
            return;
        }

        let selected_now = match self.state.selected() {
            Some(i) => i,
            None => 0,
        };

        let lenght = self.record_manager.get_all().len();
        let mut select_next = 0;
        if selected_now == 0 {
            select_next = lenght - 1;
        } else {
            select_next = selected_now - 1;
        }

        self.state.select(Some(select_next));
    }
 /// Pridá nový záznam alebo aktualizuje existujúci podľa `select_num`.
    ///
    /// # Argumenty
    ///
    /// * `input` - Vektor obsahujúci údaje záznamu vo formáte `[amount, money_type, expense, time]`.
    /// * `select_num` - Index existujúceho záznamu. Ak je -1, vytvorí sa nový záznam.
    ///
    /// # Návratová hodnota
    ///
    /// Vracia `true`, ak bol záznam úspešne spracovaný, inak `false`.
    pub async fn add_record_from_input_or_update(
        &mut self,
        input: Vec<String>,
        select_num: i32,
    ) -> bool {
        let amount: f32 = match input[0].trim().parse() {
            Ok(a) => a,
            Err(_) => return false,
        };

        let money_type1 = match input[1].trim() {
            "+" => MoneyType::INCOME,
            "-" => MoneyType::EXPENSE,
            _ => return false,
        };

        let expanse = match input[2].trim() {
            "FUN" => Some(ExpenseType::FUN),
            "RESTAURANT" => Some(ExpenseType::RESTAURANT),
            "SHOPPING" => Some(ExpenseType::SHOPPING),
            "INVESTMENT" => Some(ExpenseType::INVESTMENT),
            "FREETIME" => Some(ExpenseType::FREETIME),
            "HOME" => Some(ExpenseType::HOME),
            "CLOTH" => Some(ExpenseType::CLOTH),
            "CAR" => Some(ExpenseType::CAR),
            "TRAVEL" => Some(ExpenseType::TRAVEL),
            "OTHER" => Some(ExpenseType::OTHER),
            "NONE" => None,
            _ => None,
        };

        let time = match NaiveDate::parse_from_str(&input[3], "%d.%m.%Y") {
            Ok(t) => t,
            Err(_) => return false,
        };

        if select_num != -1 {
            let record_num = select_num as usize;
            let mut change = self.record_manager.get_all()[record_num];
            change.amount = amount;
            change.expense = expanse;
            change.time = time;
            change.money_type = money_type1;
            tokio::task::spawn_blocking(move || {
                update_record(&change);
            })
            .await
            .unwrap();

            self.record_manager = tokio::task::spawn_blocking(|| load_records())
                .await
                .unwrap();
            return true;
        } else {
            tokio::task::spawn_blocking(|| renumber_records_db())
                .await
                .unwrap();

            let id = get_next_id();

            let ret = Record {
                id: id,
                money_type: money_type1,
                amount: amount,
                expense: expanse,
                time: time,
            };

            tokio::task::spawn_blocking(move || insert_record(&ret))
                .await
                .unwrap();

            self.record_manager = tokio::task::spawn_blocking(|| load_records())
                .await
                .unwrap();
            true
        }
    }
/// Odstráni vybraný záznam z databázy a obnoví zoznam záznamov.
    pub async fn remove_record(&mut self, selected: Record) {
        tokio::task::spawn_blocking(move || {
            renumber_records_db();
            delete_record(selected);
        })
        .await
        .unwrap();
        self.record_manager = tokio::task::spawn_blocking(|| load_records())
            .await
            .unwrap();
    }
}
