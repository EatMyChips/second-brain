mod calendar;
mod header;
mod rewards;
mod tasks;

use crate::components::todo::calendar::*;
use crate::components::todo::header::*;
use crate::components::todo::rewards::*;
use crate::components::todo::tasks::*;
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate};
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::rc::Rc;

const TASKS: Asset = asset!("assets/todo/todo.css");

enum ScrollState {
    Rewards,
    Daily,
    Weekly,
}

#[derive(Clone, Copy)]
pub struct AppState {
    selected_week: Signal<NaiveDate>,
    selected_day: Signal<NaiveDate>,
    selected_month: Signal<Month>,
    current_month: Signal<Month>,
    current_week: Signal<NaiveDate>,
    current_day: Signal<NaiveDate>,
    scroll_state: Signal<ScrollState>,
}

#[component]
pub fn Todo() -> Element {
    // Time signals
    let current_day = use_signal(|| Local::now().date_naive());
    let selected_day = use_signal(|| *current_day.read());
    let current_week = use_signal(|| {
        let date = *current_day.read();
        let weekday: u32 = date.weekday().num_days_from_monday();
        date - Duration::days(weekday.into())
    });
    let selected_week = use_signal(|| *current_week.read());
    let current_month = use_signal(|| Month {year: current_day.read().year(), month: current_day.read().month()});
    let selected_month = use_signal(|| current_month.read().clone());

    //Scroll state signals
    let mut scroll_state = use_signal(|| ScrollState::Daily);
    let mut scroll_page = use_signal(|| None);
    let mut scroll_position = use_signal(|| 0.0);

    let mut calendar: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut tasks: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_context_provider(|| AppState {
        selected_week,
        selected_day,
        selected_month,
        current_month,
        current_week,
        current_day,
        scroll_state,
    });

    let _ = use_resource(move || {
        let pos = *scroll_position.read();
        if pos >= 350.0 {
            scroll_state.set(ScrollState::Weekly);
        } else if pos < 350.0 {
            scroll_state.set(ScrollState::Daily);
        }
        async move {}
    });

    let _ = use_resource(move || {
        match *scroll_state.read() {
            ScrollState::Rewards => {}
            ScrollState::Daily => {
                if let Some(page) = &*tasks.read() {
                    page.as_web_event().set_class_name("page daily");
                    page.as_web_event().set_scroll_left(0);
                }
                if let Some(page) = &*calendar.read() {
                    page.as_web_event().set_class_name("page daily");
                }
            }
            ScrollState::Weekly => {
                if let Some(page) = &*tasks.read() {
                    page.as_web_event().set_class_name("page weekly");
                }
                if let Some(page) = &*calendar.read() {
                    page.as_web_event().set_class_name("page weekly");
                }
            }
        }
        async move {}
    });

    rsx! {
        document::Stylesheet { href: TASKS }

        Header {}
        //WeeklyTaskSwitcher {  }

        div {
            class: "main-scroll",
            id: "scroll",
            onmounted: move |element| async move {
                let _ = element.set_focus(true).await;
                scroll_page.set(Some(element.data))
            },
            onscroll: move |_| async move{
                if let Some(page) = scroll_page() {
                    let scroll_pos = page.as_web_event().scroll_left() as f64;
                    scroll_position.set(scroll_pos);
                }
            },
            div {
                class: "page daily",
                id: "rewards",
                Checks {}
            }
            Calendar {calendar}
            div {
                class: "page daily",
                id: "tasks",
                onmounted: move |element|  {
                    tasks.set(Some(element.data))
                },
                List {
                    id: "todays-tasks",
                    title: "Today's Tasks"
                }
                List {
                    id: "professional",
                    title: "Professional"
                }
                List {
                    id: "personal",
                    title: "Personal"
                }
                List {
                    id: "task",
                    title: "Task"
                }
            }
        }
    }
}
