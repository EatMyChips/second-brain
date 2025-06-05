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
    let mut scroll_position = use_signal(|| 0.0);

    //Scroll  objects
    let mut calendar = use_signal(|| None);


    use_context_provider(|| AppState {
        selected_week,
        selected_day,
        current_week,
        current_day,
    });

    let _ = use_resource(move || {
        let pos = *scroll_position.read();
        async move{
        }
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
                div {
                    class: "reward",
                }
                Checks {}
            }
            div {
                class: "page",
                id: "calendar",
                 onmounted: move |element| async move {
                    let _ = element.set_focus(true).await;
                    calendar.set(Some(element.data))
                },
                Calendar {}
            }
            div {
                class: "page",
                id: "todays",
                List {
                    id: "todays-tasks",
                    title: "Today's Tasks"
                }
            }
            div {
                class: "page",
                id: "weekly",
                List {
                    id: "professional",
                    title: "Professional"
                }
                List {
                    id: "personal",
                    title: "Personal"
                }
            }
            div {
                class: "page",
                id: "tasks",
                List {
                    id: "task",
                    title: "Task"
                }
            }
        }
    }
}

#[component]
fn Calendar() -> Element {
    rsx!{
        div {
            class: "calendar",
        }
    }
}

#[component]
fn Checks() -> Element {
    rsx!{
        div {
            class: "checks",
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
                class: "nav-button",
                "Rewards"
            }
             button {
                class: "nav-button selected",
                "Daily"
            }
             button {
                class: "nav-button",
                "Weekly"
            }
        }
    }
}