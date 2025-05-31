use chrono::{DateTime, Datelike, Duration, Local};
use dioxus::prelude::*;
use crate::components::todo::weekly::TimeState;

#[component]
pub fn WeeklyTaskSwitcher() -> Element{
    let current_week = use_context::<TimeState>().current_week;
    let mut selected_week = use_context::<TimeState>().selected_week;

    let formatted_date = use_memo(move || selected_week.read().format("%d/%m/%Y").to_string());
    let week_end = use_memo(move || (*selected_week.read() + Duration::days(6)).format("%d/%m/%Y").to_string());

    rsx!{
        button {
            onclick: move |_| {
                let current = *selected_week.read();
                selected_week.set(current - Duration::days(7));
            },
            "<-"
        }
        h3 {
            "{formatted_date} - {week_end}"
        }
        if *selected_week.read() == *current_week.read() {
            h3 {
                "(current)"
            }
        }
        button {
            onclick: move |_| {
                let current = *selected_week.read();
                selected_week.set(current + Duration::days(7));
                log::info!("{}", selected_week.read());
            },
            "->",
        }
    }
}