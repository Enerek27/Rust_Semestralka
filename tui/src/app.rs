use crate::{
    event::{AppEvent, Event, EventHandler},
    record_list::RecordLister,
};
use color_eyre::eyre::Ok;
use financial_lib::{
    db::{delete_record, load_records, renumber_records_db},
    record::RecordManager,
};
use futures::future::ok;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

#[derive(Debug, PartialEq)]
pub enum FocusedWidget {
    Records,
    PieChart,
    LineChart,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub focusing_widget: FocusedWidget,
    pub record_lister: RecordLister,
    /// Event handler.
    pub events: EventHandler,

    //input mode
    pub input_mode: bool,
    pub input_select: usize,
    pub input_buffer: Vec<String>,
    //pub records: RecordManager,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            focusing_widget: FocusedWidget::Records,
            events: EventHandler::new(),
            // records: load_records(),
            record_lister: RecordLister::new(),
            input_mode: false,
            input_select: 0,
            input_buffer: vec!["".to_string(); 4],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
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
                    AppEvent::RemoveRecord => self.remove_selected_record(),
                    AppEvent::UpdateRecord => todo!(),
                    AppEvent::AddRecord => todo!(),
                    AppEvent::Addchar(c) => self.char_add(c),
                    AppEvent::Remchar => self.rem_char(),
                    AppEvent::TabInput => self.tab_input(),
                    AppEvent::BackTabInput => self.BackTabInput(),
                    AppEvent::EscReset => self.EscReset(),
                    AppEvent::EnterCOnfirm => self.EnterConfirm(),
                    AppEvent::EnterInputMode => self.enter_input_mode(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
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
        } else {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Tab => self.events.send(AppEvent::IncrementWidget),
                KeyCode::BackTab => self.events.send(AppEvent::DecrementWidget),
                KeyCode::Up => self.events.send(AppEvent::IncrementRecords),
                KeyCode::Down => self.events.send(AppEvent::DecrementRecords),
                KeyCode::Delete => self.events.send(AppEvent::RemoveRecord),
                KeyCode::Char('a') => self.events.send(AppEvent::EnterInputMode),

                // Other handlers you could add here.
                _ => {}
            }
            Ok(())
        }
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn enter_input_mode(&mut self) {
        if self.focusing_widget != FocusedWidget::Records {
            return;
        }
        self.input_mode = true;
    }

    pub fn EnterConfirm(&mut self) {
        if !self
            .record_lister
            .add_record_from_input(self.input_buffer.clone())
        {
            println!("Chyba pridania zle zadane parametre");
        };
        self.EscReset();
    }

    pub fn EscReset(&mut self) {
        self.input_buffer.iter_mut().for_each(|i| i.clear());
        self.input_select = 0;
        self.input_mode = false;
    }

    pub fn tab_input(&mut self) {
        if self.input_select == 3 {
            self.input_select = 0;
        } else {
            self.input_select += 1;
        }
    }

    pub fn BackTabInput(&mut self) {
        if self.input_select == 0 {
            self.input_select = 3;
        } else {
            self.input_select -= 1;
        }
    }

    pub fn rem_char(&mut self) {
        self.input_buffer[self.input_select].pop();
    }

    pub fn char_add(&mut self, c: char) {
        self.input_buffer[self.input_select].push(c);
    }

    pub fn increment_widget(&mut self) {
        self.focusing_widget = match self.focusing_widget {
            FocusedWidget::Records => FocusedWidget::PieChart,
            FocusedWidget::PieChart => FocusedWidget::LineChart,
            FocusedWidget::LineChart => FocusedWidget::Records,
        };
    }

    pub fn decrement_widget(&mut self) {
        self.focusing_widget = match self.focusing_widget {
            FocusedWidget::Records => FocusedWidget::LineChart,
            FocusedWidget::PieChart => FocusedWidget::Records,
            FocusedWidget::LineChart => FocusedWidget::PieChart,
        };
    }

    pub fn record_check_increment(&mut self) {
        if self.focusing_widget != FocusedWidget::Records {
            return;
        }
        self.record_lister.select_next();
    }

    pub fn record_check_decrement(&mut self) {
        if self.focusing_widget != FocusedWidget::Records {
            return;
        }
        self.record_lister.select_previous();
    }

    pub fn remove_selected_record(&mut self) {
        if self.focusing_widget != FocusedWidget::Records {
            return;
        }

        let selected = self.record_lister.state.selected();
        let selected = match selected {
            Some(s) => s,
            None => return,
        };

        let selected = self.record_lister.record_manager.get_all()[selected];
        delete_record(selected);
        renumber_records_db();
        self.record_lister.record_manager = load_records();
    }
}
