use std::ops::Deref;
use std::rc::Rc;
use dioxus::prelude::*;
use crate::backend::props::Task;
use super::{AppState, ScrollState};
use dioxus::web::WebEventExt;

const LISTS: Asset = asset!("/assets/todo/tasks.css");

#[derive(PartialEq, Props, Clone)]
pub struct ListProps {
    id: String,
    title: String,
}

#[component]
pub fn List(props: ListProps) -> Element {
    // Date signals
    let selected_week = use_context::<AppState>().selected_week;
    let selected_day = use_context::<AppState>().selected_day;
    let scroll_state = use_context::<AppState>().scroll_state;

    let mut task_bar: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let mut tasks = use_signal(|| vec!());

    // Props data
    let id = props.id.clone();

    // Get todo if selected_week or selected_day updates
    let tasks_loading = use_resource(move || {
        let id: String = props.id.clone();

        // get current dates
        let day = if id == "todays-todo" {
            Some(selected_day.read().format("%d/%m/%Y").to_string())
        } else {
            None
        };
        let week: String = selected_week.read().format("%d/%m/%Y").to_string();

        // The closure must return an async block
        async move {
            log::info!("{week:?},{day:?}");
            tasks.set(Task::get_all(id, week, day).await);
        }
    });

    let _ = use_resource(move || {
        match *scroll_state.read() {
            ScrollState::Rewards =>  {

            },
            ScrollState::Daily =>  {
                if let Some(page) = &*task_bar.read() {
                    page.as_web_event().set_class_name("element daily")
                }
            },
            ScrollState::Weekly =>  {
                if let Some(page) = &*task_bar.read() {
                    page.as_web_event().set_class_name("element weekly")
                }
            },
        }
        async move{
        }
    });

    rsx!{
        match tasks_loading.read_unchecked().deref() {
            Some(_) => {
                rsx! {
                    document::Stylesheet { href: LISTS}

                    div{
                        class: "element daily",
                        id: id.clone(),
                        tabindex: "0",
                        onmounted: move |element|  {
                            task_bar.set(Some(element.data))
                        },
                        onkeydown: move |event: Event<KeyboardData>| {
                            let id = id.clone();
                            async move {
                                let key = event.data.key();
                                if key == Key::Enter {
                                    let day = if id == "todays-todo" {
                                        Some(selected_day.read().format("%d/%m/%Y").to_string())
                                    } else {
                                        None
                                    };
                                    let week = Some(selected_week.read().format("%d/%m/%Y").to_string());

                                    tasks.write().push(Task::new(week, day, id).await);
                                }
                            }
                        },
                        ListHeader {
                            title: props.title,
                            id: id.clone(),
                        }
                        div{
                            class: "tasks",
                            for id in tasks.read().clone() {
                                TaskComp {id},
                            }
                        }
                    }
                }
            },
            None =>  rsx! {
                div{
                    class: "element list",
                    id: id.clone(),
                    tabindex: "0",
                    ListHeader {
                        title: props.title,
                        id: id.clone(),
                    },
                    h3 {
                        "Loading..."
                    }
                }
            }
        }
    }
}

#[component]
fn TaskComp(id: i64) -> Element{
    let task = use_future(move || {
        let id = id.clone();
        async move {
            Task::get(id).await
        }
    });

    rsx!{
        div {
            class: "task",
            input {
                class: "check",
                type: "checkbox",
            }
            input {
                class: "task-heading",
                value: "SECOND-BRAIN - do the styling",
                tabindex: "0",
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct ListHeaderProps {
    title: String,
    id: String,
}

#[component]
fn ListHeader(props: ListHeaderProps) -> Element {
    rsx!{
        div{
            class: "header",
            h2 { {props.title} }

            // if props.id == "todays-todo"{
            //     DailyTaskSwitcher { }
            // }
        }
    }
}

fn string_split(input: String) -> Vec<String> {
    if input.contains('~') {
        input
            .split('~')
            .map(|s| s.to_string())
            .collect()
    } else {
        vec!["".to_string(), input]
    }
}