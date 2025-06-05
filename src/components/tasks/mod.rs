mod lists;
mod date_features;

use std::ops::Deref;
use dioxus::prelude::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use chrono::{DateTime, Datelike, Duration, Local};
use dioxus::web::WebEventExt;
use crate::components::tasks::lists::*;
use crate::components::tasks::date_features::*;

const HEADER: Asset = asset!("assets/tasks/header.css");
const TASKS: Asset = asset!("assets/tasks/tasks.css");

#[derive(Clone, Copy)]
pub struct AppState {
    selected_week: Signal<DateTime<Local>>,
    selected_day: Signal<DateTime<Local>>,
    current_week: Signal<DateTime<Local>>,
    current_day: Signal<DateTime<Local>>,
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

    //Scroll state signals
    let mut scroll_page = use_signal(|| None);
    let mut rewards = use_signal(|| None);
    let mut today = use_signal(|| None);
    let mut weekly = use_signal(|| None);
    let mut task = use_signal(|| None);
    let mut scroll_position = use_signal(|| 0.0);

    use_context_provider(|| AppState {
        selected_week,
        selected_day,
        current_week,
        current_day,
    });

    rsx!{
        document::Stylesheet { href: TASKS}

        Header {}
        //WeeklyTaskSwitcher {  }

        div {
            class: "weekly-lists",
            id: "scroll",
            onmounted: move |element| async move {
                let _ = element.set_focus(true).await;
                scroll_page.set(Some(element.data))
            },
            onscroll: move |_| async move{
                if let Some(page) = scroll_page() {
                    let scroll_pos = page.as_web_event().scroll_left() as f64;
                    scroll_position.set(scroll_pos);
                    log::info!("{scroll_pos}");
                }
            },
            div {
                class: "page",
                id: "rewards",
                onmounted: move |element| rewards.set(Some(element.data)),
            },
            div {
                class: "page",
                id: "today",
                onmounted: move |element| today.set(Some(element.data)),
                List {
                    id: String::from("todays-tasks"),
                    title: String::from("Today's Tasks"),
                }
            }
            div {
                class: "page",
                id: "weekly",
                onmounted: move |element| weekly.set(Some(element.data)),
                List {
                    id: String::from("professional"),
                    title: String::from("Professional"),
                }
                List {
                    id: String::from("personal"),
                    title: String::from("Personal"),
                }
            },
            div {
                class: "page",
                id: "task",
                onmounted: move |element| task.set(Some(element.data)),
                List {
                    id: String::from("tasks"),
                    title: String::from("Tasks"),
                }
                List {
                    id: String::from("deadlines"),
                    title: String::from("Deadlines"),
                }
            },
        }
    }
}

#[component]
pub fn Header() -> Element{
    rsx!{
        document::Stylesheet { href: HEADER}
        div {
            class: "nav-bar",
             button {
                class: "nav-button selected",
                "data-index": "0",
                "Daily"
            }
             button {
                class: "nav-button",
                "data-index": "1",
                "Weekly"
            }
             button {
                class: "nav-button",
                "data-index": "2",
                "Tasks"
            }
        }
    }
}