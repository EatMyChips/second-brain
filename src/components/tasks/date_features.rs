use chrono::{DateTime, Datelike, Duration, Local};
use dioxus::prelude::*;
use crate::components::tasks::AppState;

const DATES: Asset = asset!("/assets/tasks/weekly/date_features.css");

#[component]
pub fn WeeklyTaskSwitcher() -> Element{
    let current_week = use_context::<AppState>().current_week;
    let mut selected_week = use_context::<AppState>().selected_week;
    let mut selected_day = use_context::<AppState>().selected_day;
    let current_day = use_context::<AppState>().current_day;

    let formatted_date = use_memo(move || selected_week.read().format("%d/%m/%Y").to_string());
    let week_end = use_memo(move || (*selected_week.read() + Duration::days(6)).format("%d/%m/%Y").to_string());

    rsx!{
        document::Stylesheet { href: DATES}

        div {
            class: "time-object",
            button {
                class: "switch-time",
                onclick: move |_| {
                    let current = *selected_week.read();
                    selected_week.set(current - Duration::days(7));
                    if formatted_date.to_string() == *current_week.read().format("%d/%m/%Y").to_string() {
                        selected_day.set(*current_day.read());
                    }
                    else{
                        selected_day.set(*selected_week.read());
                    }
                },
                "<-"
            }
            h3 {
                class: "display-date",
                "{formatted_date} - {week_end}"
            }
            if formatted_date.to_string() == *current_week.read().format("%d/%m/%Y").to_string() {
                h3 {
                    class: "display-date",
                    "(current)"
                }
            }
            button {
                class: "switch-time",
                onclick: move |_| {
                    let current = *selected_week.read();
                    selected_week.set(current + Duration::days(7));
                    if formatted_date.to_string() == *current_week.read().format("%d/%m/%Y").to_string() {
                        selected_day.set(*current_day.read());
                    }
                    else{
                        selected_day.set(*selected_week.read());
                    }
                },
                "->",
            }
        }
    }
}

#[component]
pub fn DailyTaskSwitcher() -> Element{
    let current_day = use_context::<AppState>().current_day;
    let mut selected_day = use_context::<AppState>().selected_day;
    let mut selected_week = use_context::<AppState>().selected_week;

    let formatted_date = use_memo(move || selected_day.read().format("%d/%m/%Y").to_string());
    let week_end = use_memo(move || *selected_week.read() + Duration::days(6));

    rsx!{
        document::Stylesheet { href: DATES}

        div {
            class: "time-object",
            button {
                class: "switch-time",
                onclick: move |_| {
                    if *selected_day.read() != *selected_week.read() {
                        let current = *selected_day.read();
                        selected_day.set(current - Duration::days(1));
                    }
                },
                "<-"
            }
            h3 {
                class: "display-date",
                "{formatted_date}"
            }
            if formatted_date.to_string() == *current_day.read().format("%d/%m/%Y").to_string() {
                h3 {
                    class: "display-date",
                    "(today)"
                }
            }
            button {
                class: "switch-time",
                onclick: move |_| {
                    if *selected_day.read() != *week_end.read() {
                        let current = *selected_day.read();
                        selected_day.set(current + Duration::days(1));
                    }
                },
                "->",
            }
        }
    }
}