use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::symbols::{self, block};
use ratatui::widgets::{Axis, BarChart, Borders, Chart, Dataset, List, ListItem, StatefulWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::{App, FocusedWidget};
use crate::chart_calculator;
use crate::chart_calculator::chart_calculator::{data_for_time_graph, percentage_for_pie};

const SELECTED: Style = Style::new()
    .bg(Color::LightMagenta)
    .add_modifier(Modifier::BOLD);

impl App {
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

        let data= percentage_for_pie(&self.record_lister);
    
       
        let mut border = Block::bordered()
            .title("Expenses")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if self.focusing_widget == FocusedWidget::PieChart {
            border = border.border_style(Style::new().bg(Color::LightCyan));
        }

        

        let chart = BarChart::default()
        .block(border)
        .data(&data)
        .bar_width(5)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::LightRed))
        .value_style(Style::default().fg(Color::Black).bg(Color::LightRed).add_modifier(Modifier::BOLD));

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
        let dataset = Dataset::default().marker(symbols::Marker::Dot)
        .style(Style::default().bg(Color::LightYellow)).data(&data);


        let chart = Chart::new().block(border).x_axis(
            Axis::default()
            .bounds([data.first().map(|x ,_| x.clone()).unwrap()])


        )


        
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
    }
}
