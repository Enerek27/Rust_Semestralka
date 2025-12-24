pub mod chart_calculator {
    use chrono::{DateTime, naive::NaiveDate};
    use financial_lib::record::{ExpenseType, Record};
    use ratatui::{layout::Rect, style::Color, text::Span};
    use std::collections::BTreeMap;

    use crate::record_list::RecordLister;

    pub fn color_for_category(category: &ExpenseType) -> Color {
        match category {
            ExpenseType::FUN => Color::Cyan,
            ExpenseType::HOME => Color::Green,
            ExpenseType::CAR => Color::Red,
            ExpenseType::SHOPPING => Color::Magenta,
            ExpenseType::TRAVEL => Color::Yellow,
            ExpenseType::INVESTMENT => Color::Blue,
            ExpenseType::RESTAURANT => Color::LightRed,
            ExpenseType::CLOTH => Color::LightMagenta,
            ExpenseType::FREETIME => Color::LightBlue,
            ExpenseType::OTHER => Color::Gray,
        }
    }

    pub fn percentage_for_pie(record_lister: &RecordLister) -> Vec<(&str, u64)> {
        let mut ret = Vec::new();

        let original_values = record_lister.record_manager.categories_to_hash();

        for (category, value) in original_values {
            
            ret.push((category.into(), value as u64));
        }

        ret
    }

    pub fn data_for_time_graph(record_lister: &RecordLister) -> Vec<(f64, f64)> {
        let mut ret = Vec::new();
        let mut days: BTreeMap<NaiveDate, f64> = BTreeMap::new();
        let all_records = record_lister.record_manager.get_all();
        let mut balance: f64 = 0.0;

        for r in all_records {
            match r.money_type {
                financial_lib::record::MoneyType::INCOME => balance += r.amount as f64,
                financial_lib::record::MoneyType::EXPENSE => balance -= r.amount as f64,
            }

            days.insert(r.time, balance);
        }

        for (time, amount) in days {
            let insert = time
                .and_hms_opt(0, 0, 0)
                .expect("Conversion error to NaiveDateTime")
                .and_utc()
                .timestamp() as f64;
            ret.push((insert, amount));
        }

        ret
    }

    

    pub fn generate_x_labels(
        record_lister: &RecordLister,
        label_count: usize,
    ) -> Vec<Span<'static>> {
        let dates = record_lister.record_manager.get_all();
        let mut new_dates: Vec<NaiveDate> = dates.into_iter().map(|r| r.time).collect();
        new_dates.sort();
        new_dates.dedup();

        let lenght = new_dates.len();
        if lenght == 0 {
            return vec![];
        }

        if new_dates.len() <= label_count {
            return new_dates
                .into_iter()
                .map(|r| Span::from(r.format("%d.%m.%Y").to_string()))
                .collect();
        }

        let step = lenght / label_count.max(1);
        let mut ret = Vec::new();

        let mut counter = 0;

        while counter < lenght {
            ret.push(Span::from(
                new_dates[counter].format("%d.%m.%Y").to_string(),
            ));
            counter += step.max(1);
        }

        if ret.last().expect("Chyba posledneho").clone()
            != Span::from(
                new_dates
                    .last()
                    .expect("Chyba posledneho dates")
                    .format("%d.%m.%Y")
                    .to_string(),
            )
        {
            ret.push(Span::from(
                new_dates
                    .last()
                    .expect("Chyba posledneho dates")
                    .format("%d.%m.%Y")
                    .to_string(),
            ));
        }

        ret
    }
}
