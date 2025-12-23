use core::f64;

use crossterm::style;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::symbols::{self, block};
use ratatui::text::Span;
use ratatui::widgets::{Axis, BarChart, Borders, Chart, Dataset, List, ListItem, StatefulWidget};
use ratatui::{DefaultTerminal, Frame};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::{App, FocusedWidget};
use crate::chart_calculator;
use crate::chart_calculator::chart_calculator::{
    data_for_time_graph, generate_x_labels, percentage_for_pie,
};

const SELECTED: Style = Style::new()
    .bg(Color::LightMagenta)
    .add_modifier(Modifier::BOLD);

impl App {
    pub fn render_input_mode(&mut self, buf: &mut Buffer, area: Rect) {
        let windows = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(vec![Constraint::Length(3); self.input_buffer.len()])
            .split(area);

        let titles = [
            "Amount",
            "Type(+/-)",
            "Category(Fun, Restaurant, Shopping, Investment, Freetime, Home, Cloth, Car, Travel, Other)",
            "Date- dd.mm.yyyy",
        ];
       
        
        for (i, window) in windows.iter().enumerate() {
            let buffer = &self.input_buffer[i];

            let style = if i == self.input_select as usize {
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                .bg(Color::Black)
            };

        
            let paragraph = Paragraph::new(buffer.as_str()) 
                .block(
                    Block::default()
                        .title(titles[i]) 
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Black)),
                ) 
                .style(style);

            //buf.set_style(window.clone(), style);
            paragraph.render(*window, buf);
        }
    }

    pub fn render_records(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .record_lister
            .record_manager
            .format_all()
            .into_iter()
            .map(|r| ListItem::new(r))
            .collect();

        let mut border = Block::bordered()
            .title("Records")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        if self.focusing_widget == FocusedWidget::Records {
            border = border.border_style(Style::new().bg(Color::LightCyan));
        }

        if self.record_lister.state.selected().is_none()
            && !self.record_lister.record_manager.get_all().is_empty()
        {
            self.record_lister.state.select(Some(0));
        }

        let highlight_style = if self.focusing_widget == FocusedWidget::Records {
            SELECTED
        } else {
            Style::new()
        };

        let record_list = List::new(items)
            .block(border)
            .highlight_style(highlight_style)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
        StatefulWidget::render(record_list, area, buf, &mut self.record_lister.state);
    }

    pub fn render_pseudo_pie_chart(&mut self, area: Rect, buf: &mut Buffer) {
        let data = percentage_for_pie(&self.record_lister);

        let mut border = Block::bordered()
            .title("Expenses")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if self.focusing_widget == FocusedWidget::PieChart {
            border = border.border_style(Style::new().bg(Color::LightCyan));
        }

        let bars_count = data.len() as u16;
        let inner_width = area.width.saturating_sub(2);

        let bar_width = (inner_width / bars_count).saturating_sub(1);
        let bar_gap = 2;

        let chart = BarChart::default()
            .block(border)
            .data(&data)
            .bar_width(bar_width)
            .bar_gap(bar_gap)
            .bar_style(Style::default().fg(Color::LightRed))
            .value_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            );

        chart.render(area, buf);
    }

    pub fn render_balance_chart(&mut self, area: Rect, buf: &mut Buffer) {
        let mut border = Block::bordered()
            .title("Balance over time")
            .border_type(BorderType::Rounded);
        if self.focusing_widget == FocusedWidget::LineChart {
            border = border.border_style(Style::new().bg(Color::LightCyan));
        }

        let data = data_for_time_graph(&self.record_lister);
        let dataset = Dataset::default()
            .marker(symbols::Marker::Dot)
            .style(
                Style::default()
                    .bg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )
            .data(&data);

        let y_min = data
            .iter()
            .map(|(_, y)| y.clone())
            .fold(f64::INFINITY, f64::min);
        let y_max = data
            .iter()
            .map(|(_, y)| y.clone())
            .fold(f64::NEG_INFINITY, f64::max);

        let y_labels = vec![
            Span::from(format!("{:.2}", y_min)),
            Span::from(format!("{:.2}", (y_min + y_max) / 2.0)),
            Span::from(format!("{:.2}", y_max)),
        ];

        let min_label_width = 20;
        let label_count = (area.width as usize / min_label_width);

        let x_labels = generate_x_labels(&self.record_lister, label_count);

        let chart = Chart::new(vec![dataset])
            .block(border)
            .x_axis(
                Axis::default()
                    .bounds([data.first().unwrap().0, data.last().unwrap().0])
                    .labels(x_labels),
            )
            .y_axis(Axis::default().bounds([y_min, y_max]).labels(y_labels));

        chart.render(area, buf);
    }
}

impl Widget for &mut App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        let top_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_split[0]);

        self.render_records(top_split[0], buf);
        self.render_balance_chart(main_split[1], buf);
        self.render_pseudo_pie_chart(top_split[1], buf);

        if self.input_mode {
   
        let pop_up = ratatui::layout::Rect {
            x: area.x + area.width / 4,
            y: area.y + area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };
        self.render_input_mode(buf, pop_up); 
        }
    }
}
