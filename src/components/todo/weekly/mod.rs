mod lists;
mod date_features;

use dioxus::prelude::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use chrono::{DateTime, Datelike, Duration, Local};
use lists::*;
use date_features::*;

const WEEKLY: Asset = asset!("/assets/todo/weekly/weekly.css");

#[derive(Clone, Copy)]
pub struct TimeState {
    selected_week: Signal<DateTime<Local>>,
    selected_day: Signal<DateTime<Local>>,
    current_week: Signal<DateTime<Local>>,
    current_day: Signal<DateTime<Local>>,
}

#[component]
pub fn Weekly() -> Element {
    // Time signals
    let current_day = use_signal(|| Local::now() );
    let selected_day = use_signal(|| *current_day.read() );
    let current_week = use_signal(|| {
        let dt: DateTime<Local> = *current_day.read();
        let weekday: u32 = dt.weekday().num_days_from_monday();
        dt - Duration::days(weekday.into())
    });
    let selected_week = use_signal(|| *current_week.read() );

    // Page state signals
    let mut edit_mode = use_signal(|| false);

    use_context_provider(|| TimeState {
        selected_week,
        selected_day,
        current_week,
        current_day,
    });

    rsx!{
        document::Stylesheet { href: WEEKLY}

        super::Header {}
        WeeklyTaskSwitcher {  }

        button {
            class: "mode_switch",
            onclick: move |_| {
                let edit = edit_mode.read().clone();
                edit_mode.set(!edit);
            }
        }
        div {
            class: "weekly-lists",
            TodaysTasks {  }
            CheckBoxes {  }
            div { class: "break" }
            University {  }
            Personal {  }
            Life {  }
        }

    }
}

#[component]
fn TodaysTasks() -> Element {
    rsx!{
        List {
            id: String::from("todays-tasks"),
            title: String::from("Today's Tasks"),
        }
    }
}

#[component]
fn CheckBoxes() -> Element {
    rsx!{
        div{
            class: "element",
            tabindex: "0",
        }
    }
}

#[component]
fn University() -> Element {
    rsx!{
        List {
            id: String::from("university"),
            title: String::from("University"),
        }
    }
}

#[component]
fn Personal() -> Element {
    rsx!{
        List {
            id: String::from("personal"),
            title: String::from("Personal"),
        }
    }
}

#[component]
fn Life() -> Element {
    rsx!{
        List {
            id: String::from("life"),
            title: String::from("Life"),
        }
    }
}