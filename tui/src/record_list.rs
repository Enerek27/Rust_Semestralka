use financial_lib::{db::load_records, record::RecordManager};
use ratatui::widgets::ListState;

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
}
