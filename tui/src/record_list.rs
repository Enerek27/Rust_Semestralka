use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use color_eyre::owo_colors::styles::BoldDisplay;
use financial_lib::{
    db::{get_next_id, insert_record, load_records, renumber_records_db},
    record::{ExpenseType, MoneyType, Record, RecordManager},
};
use ratatui::{text::Span, widgets::ListState};

#[derive(Debug)]
pub struct RecordLister {
    pub record_manager: RecordManager,
    pub state: ListState,
}

impl RecordLister {
    pub fn new() -> Self {
        RecordLister {
            record_manager: load_records(),
            state: ListState::default(),
        }
    }

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

    pub fn add_record_from_input(&mut self, input: Vec<String>) -> bool {
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

        renumber_records_db();

        let id = get_next_id();

        let ret = Record {
            id: id,
            money_type: money_type1,
            amount: amount,
            expense: expanse,
            time: time,
        };

        insert_record(&ret);
        self.record_manager = load_records();
        true
    }
}
