use crate::{
    event::{AppEvent, Event, EventHandler},
    record_list::RecordLister,
};
use financial_lib::{db::load_records, record::RecordManager};
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

    pub records: RecordManager,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            focusing_widget: FocusedWidget::Records,
            events: EventHandler::new(),
            records: load_records(),
            record_lister: RecordLister::new(),
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
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Tab => self.events.send(AppEvent::IncrementWidget),
            KeyCode::BackTab => self.events.send(AppEvent::DecrementWidget),
            KeyCode::Up => self.events.send(AppEvent::IncrementRecords),
            KeyCode::Down => self.events.send(AppEvent::DecrementRecords),

            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
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
}
