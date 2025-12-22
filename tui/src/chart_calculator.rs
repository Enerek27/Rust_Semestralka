pub mod chart_calculator {
    use std::collections::BTreeMap;

    use financial_lib::record::ExpenseType;
    use ratatui::{layout::Rect, style::Color};

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

    pub fn percentage_for_pie(
        record_lister: &RecordLister,
    ) -> Vec<(&str, u64)> {
        let mut ret = Vec::new();


        let original_values = record_lister.record_manager.categories_to_hash();


        for (category, value) in original_values {

            ret.push((category.into(), value as u64));
        }

        ret
    }


    pub fn data_for_time_graph(record_lister: &RecordLister) -> Vec<(f64, f64)> {
        let mut ret = Vec::new();
        let all_records = record_lister.record_manager.get_all();
        let mut balance: f64 = 0.0;

        for r in all_records {
            match r.money_type {
                financial_lib::record::MoneyType::INCOME => balance += r.amount as f64,
                financial_lib::record::MoneyType::EXPENSE => balance -= r.amount as f64,
            }
            let seconds_from = r.time.and_hms_opt(0, 0, 0).expect("Conversion error to NaiveDateTime");
            let seconds_from = seconds_from.and_utc().timestamp() as f64;
            ret.push((seconds_from, balance));
        }

        ret
    }

}
