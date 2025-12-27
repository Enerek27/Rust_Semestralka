
//! Hlavný aplikačný modul TUI aplikácie.
use crate::{
    event::{AppEvent, Event, EventHandler},
    record_list::RecordLister,
};
use color_eyre::eyre::Ok;
use financial_lib::record::{ExpenseType, Record};

use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent},
};

/// Určuje, ktorý widget je momentálne aktívny.
#[derive(Debug, PartialEq)]
pub enum FocusedWidget {
    Records,
    PieChart,
    LineChart,
}

/// Aplikácia.
#[derive(Debug)]
pub struct App {
    /// beží aplikácia
    pub running: bool,
    /// Ktorý widget je aktívny
    pub focusing_widget: FocusedWidget,
    /// správca záznamov
    pub record_lister: RecordLister,
    /// srpáva handlara
    pub events: EventHandler,
    /// je zobrazený help
    pub help_show: bool,

    ///input mode zapnutý
    pub input_mode: bool,
    pub input_select: usize,
    /// čo sa má zobrazit v input režime
    pub input_buffer: Vec<String>,
    
    ///update mode zapnutý
    pub update_mode: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            focusing_widget: FocusedWidget::Records,
            events: EventHandler::new(),
            record_lister: RecordLister::new(),
            input_mode: false,
            input_select: 0,
            input_buffer: vec!["".to_string(); 4],
            update_mode: false,
            help_show: false,
        }
    }
}

impl App {
    /// vytvortý novú inštanciu [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Hlavný cyklus applikácie
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::DecrementWidget => self.decrement_widget(),
                    AppEvent::IncrementWidget => self.increment_widget(),
                    AppEvent::IncrementRecords => self.record_check_decrement(),
                    AppEvent::DecrementRecords => self.record_check_increment(),
                    AppEvent::RemoveRecord => self.remove_selected_record().await,
                    AppEvent::UpdateRecord => todo!(),
                    AppEvent::AddRecord => todo!(),
                    AppEvent::Addchar(c) => self.char_add(c),
                    AppEvent::Remchar => self.rem_char(),
                    AppEvent::TabInput => self.tab_input(),
                    AppEvent::BackTabInput => self.BackTabInput(),
                    AppEvent::EscReset => self.EscReset(),
                    AppEvent::EnterCOnfirm => self.EnterConfirm().await,
                    AppEvent::EnterInputMode => self.enter_input_mode(),
                    AppEvent::EditRecord => self.enter_edit_mode(),
                    AppEvent::HelpEnter => self.help_enter(),
                    AppEvent::HelpExit => self.help_exit(),
                },
            }
        }
        Ok(())
    }

    /// handler pre eventy  [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if self.input_mode {
            match key_event.code {
                KeyCode::Char(c) => self.events.send(AppEvent::Addchar(c)),
                KeyCode::Backspace => self.events.send(AppEvent::Remchar),
                KeyCode::Tab => self.events.send(AppEvent::TabInput),
                KeyCode::BackTab => self.events.send(AppEvent::BackTabInput),
                KeyCode::Esc => self.events.send(AppEvent::EscReset),
                KeyCode::Enter => self.events.send(AppEvent::EnterCOnfirm),
                _ => {}
            }
            Ok(())
        } else if self.help_show {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::HelpExit),

                _ => {}
            }
            Ok(())
        } else {
            match key_event.code {
                KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Tab => self.events.send(AppEvent::IncrementWidget),
                KeyCode::BackTab => self.events.send(AppEvent::DecrementWidget),
                KeyCode::Up => self.events.send(AppEvent::IncrementRecords),
                KeyCode::Down => self.events.send(AppEvent::DecrementRecords),
                KeyCode::Delete => self.events.send(AppEvent::RemoveRecord),
                KeyCode::Char('a') => self.events.send(AppEvent::EnterInputMode),
                KeyCode::Enter => self.events.send(AppEvent::EditRecord),
                KeyCode::Char('h') => self.events.send(AppEvent::HelpEnter),

                
                _ => {}
            }
            Ok(())
        }
    }

     /// Aktualizácia stavu aplikácie pri tick udalosti.
    pub fn tick(&self) {}

    /// Ukončí aplikáciu.
    pub fn quit(&mut self) {
        self.running = false;
    }
 /// Aktivuje zobrazenie pomoci
    pub fn help_enter(&mut self) {
        self.help_show = true;
    }
/// vypne zobrazenie pomoci
    pub fn help_exit(&mut self) {
        self.help_show = false;
    }
   /// Prepne aplikáciu do režimu úpravy záznamu.
    pub fn enter_edit_mode(&mut self) {
        if self.focusing_widget != FocusedWidget::Records
            || self.record_lister.record_manager.get_all().len() == 0
        {
            return;
        }
        let record = self
            .record_lister
            .state
            .selected()
            .expect("No selected error in enter_edit_mode");
        let record = self.record_lister.record_manager.get_all()[record];
        self.input_buffer = record_to_edit_mode(&record);
        self.update_mode = true;
        self.input_mode = true;
    }
  /// Aktivuje režim pridávania nového záznamu.
    pub fn enter_input_mode(&mut self) {
        if self.focusing_widget != FocusedWidget::Records {
            return;
        }

        self.input_mode = true;
    }
 /// Potvrdí zadanie alebo úpravu záznamu.
    pub async fn EnterConfirm(&mut self) {
        let success = if self.update_mode {
            let selected_index = self
                .record_lister
                .state
                .selected()
                .expect("Nothing selected") as i32;

            self.record_lister
                .add_record_from_input_or_update(self.input_buffer.clone(), selected_index)
                .await
        } else {
            self.record_lister
                .add_record_from_input_or_update(self.input_buffer.clone(), -1)
                .await
        };

        if !success {
            println!("Error: wrong parameters");
        }

        self.EscReset();
    }
/// Resetuje vstupný režim.
    pub fn EscReset(&mut self) {
        self.input_buffer.iter_mut().for_each(|i| i.clear());
        self.input_select = 0;
        self.input_mode = false;
        self.update_mode = false;
    }
/// Posunie kurzor na ďalšie vstupné pole.
    pub fn tab_input(&mut self) {
        if self.input_select == 3 {
            self.input_select = 0;
        } else {
            self.input_select += 1;
        }
    }
 /// Posunie kurzor na predchádzajúce vstupné pole.
    pub fn BackTabInput(&mut self) {
        if self.input_select == 0 {
            self.input_select = 3;
        } else {
            self.input_select -= 1;
        }
    }
 /// Odstráni posledný znak zo vstupu.
    pub fn rem_char(&mut self) {
        self.input_buffer[self.input_select].pop();
    }
  /// Pridá znak do vstupu.
    pub fn char_add(&mut self, c: char) {
        self.input_buffer[self.input_select].push(c);
    }
/// Presunie fokus na ďalší widget.
    pub fn increment_widget(&mut self) {
        self.focusing_widget = match self.focusing_widget {
            FocusedWidget::Records => FocusedWidget::PieChart,
            FocusedWidget::PieChart => FocusedWidget::LineChart,
            FocusedWidget::LineChart => FocusedWidget::Records,
        };
    }
/// Presunie fokus na predchádzajúci widget.
    pub fn decrement_widget(&mut self) {
        self.focusing_widget = match self.focusing_widget {
            FocusedWidget::Records => FocusedWidget::LineChart,
            FocusedWidget::PieChart => FocusedWidget::Records,
            FocusedWidget::LineChart => FocusedWidget::PieChart,
        };
    }

    /// Posunie výber záznamov nahor.
  
    pub fn record_check_increment(&mut self) {
        if self.focusing_widget != FocusedWidget::Records {
            return;
        }
        self.record_lister.select_next();
    }
/// Posunie výber záznamov nadol.
    pub fn record_check_decrement(&mut self) {
        if self.focusing_widget != FocusedWidget::Records {
            return;
        }
        self.record_lister.select_previous();
    }
/// Odstráni vybraný záznam.
    pub async fn remove_selected_record(&mut self) {
        if self.focusing_widget != FocusedWidget::Records {
            return;
        }

        let selected = self.record_lister.state.selected();
        let selected = match selected {
            Some(s) => s,
            None => return,
        };

        let selected = self.record_lister.record_manager.get_all()[selected];
        self.record_lister.remove_record(selected).await;
    }
}
/// Konvertuje záznam do formátu pre editáciu záznamu
pub fn record_to_edit_mode(record: &Record) -> Vec<String> {
    let amount = format!("{:.2}", record.amount);

    let money_type = match record.money_type {
        financial_lib::record::MoneyType::INCOME => "+".to_string(),
        financial_lib::record::MoneyType::EXPENSE => "-".to_string(),
    };

    let expense = match record.expense {
        Some(e) => match e {
            ExpenseType::FUN => "FUN".to_string(),
            ExpenseType::RESTAURANT => "RESTAURANT".to_string(),
            ExpenseType::SHOPPING => "SHOPPING".to_string(),
            ExpenseType::INVESTMENT => "INVESTMENT".to_string(),
            ExpenseType::FREETIME => "FREETIME".to_string(),
            ExpenseType::HOME => "HOME".to_string(),
            ExpenseType::CLOTH => "CLOTH".to_string(),
            ExpenseType::CAR => "CAR".to_string(),
            ExpenseType::TRAVEL => "TRAVEL".to_string(),
            ExpenseType::OTHER => "OTHER".to_string(),
        },
        None => "NONE".to_string(),
    };

    let time = record.time.format("%d.%m.%Y").to_string();

    vec![amount, money_type, expense, time]
}
