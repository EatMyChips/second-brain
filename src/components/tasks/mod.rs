mod lists;
mod date_features;

use dioxus::prelude::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use chrono::{DateTime, Datelike, Duration, Local};
use crate::components::tasks::lists::*;
use crate::components::tasks::date_features::*;

const TODO: Asset = asset!("assets/tasks/todo.css");
const TASKS: Asset = asset!("assets/tasks/weekly/tasks.css");

#[derive(Clone, Copy)]
pub struct AppState {
    selected_week: Signal<DateTime<Local>>,
    selected_day: Signal<DateTime<Local>>,
    current_week: Signal<DateTime<Local>>,
    current_day: Signal<DateTime<Local>>,
    edit_mode: Signal<bool>,
}

#[component]
pub fn Tasks() -> Element {
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

    use_context_provider(|| AppState {
        selected_week,
        selected_day,
        current_week,
        current_day,
        edit_mode,
    });

    rsx!{
        document::Stylesheet { href: TASKS}

        Header {}
        //WeeklyTaskSwitcher {  }
        TodaysTasks {  }
        div {
            class: "weekly-lists",
            /*CheckBoxes {  }*/
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

#[component]
pub fn Header() -> Element{
    rsx!{
        document::Stylesheet { href: TODO}
        div {
            class: "nav-bar",
             button {
                class: "nav-button selected",
                "Tasks"
            }
             button {
                class: "nav-button",
                "Rewards"
            }
             button {
                class: "nav-button",
                id: "rewards",
                "Finance"
            }
        }
    }
}