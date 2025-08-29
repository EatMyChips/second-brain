use super::{AppState, ScrollState};
use crate::backend::props::Task;
use crate::backend::frontend_link::tasks_link::{update, new, get, get_all, delete, update_completed};
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::ops::Deref;
use std::rc::Rc;
use dioxus::html::completions::CompleteWithBraces::feTile;
use crate::props;

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

    let mut tasks = use_signal(|| vec![]);

    // Props data
    let id = props.id.clone();

    // Get todo if selected_week or selected_day updates
    let tasks_loading = use_resource(move || {
        let id: String = props.id.clone();

        // get current dates
        let day = if id == "todays-tasks" {
            Some(selected_day.read().format("%d/%m/%Y").to_string())
        } else {
            None
        };
        let week: String = selected_week.read().format("%d/%m/%Y").to_string();

        // gets all task data
        async move {
            tasks.set(get_all(id, week, day).await);
        }
    });

    let _ = use_resource(move || {
        match *scroll_state.read() {
            ScrollState::Rewards => {}
            ScrollState::Daily => {
                if let Some(page) = &*task_bar.read() {
                    page.as_web_event().set_class_name("element daily")
                }
            }
            ScrollState::Weekly => {
                if let Some(page) = &*task_bar.read() {
                    page.as_web_event().set_class_name("element weekly")
                }
            }
        }
        async move {}
    });

    rsx! {
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
                                    let day = if id == "todays-tasks" {
                                        Some(selected_day.read().format("%d/%m/%Y").to_string())
                                    } else {
                                        None
                                    };
                                    let week = Some(selected_week.read().format("%d/%m/%Y").to_string());

                                    tasks.write().push(new(week, day, id).await);
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
                                TaskComp {id, tasks},
                            }
                        }
                    }
                }
            },
            None =>  rsx! {
                document::Stylesheet { href: LISTS}
                div{
                    class: "element daily",
                    id: id.clone(),
                    tabindex: "0",
                    ListHeader {
                        title: props.title,
                        id: id.clone(),
                    },
                    div {
                        class: "task",
                        h3 {
                            "Loading..."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TaskComp(id: i64, tasks: Signal<Vec<i64>>) -> Element {
    let mut task_text = use_signal(|| "".to_string());
    let mut task_title = use_signal(|| "".to_string());
    let mut task_completed = use_signal(|| false);

    let loaded = use_resource(move || {
        let id = id.clone();
        async move {
            let fetched_task = get(id).await;
            task_text.set(fetched_task.info.clone());
            task_title.set(fetched_task.title.clone());
            task_completed.set(fetched_task.completed.clone());
        }
    });

    if let Some(_) = loaded.read_unchecked().deref() {
        let _ = use_resource(move || {
            let task_text = task_text.read().clone();

            async move {
                update(id, task_text).await;
            }
        });

        let _ = use_resource(move || {
            let task_completed = task_completed.read().clone();

            async move {
                update_completed(id, task_completed).await;
            }
        });
    }


    rsx! {
        div {
            class: "task",
            input {
                class: "check",
                type: "checkbox",
                checked: *task_completed.read(),
                onchange: move |evt| {
                    task_completed.set(evt.value() == "true");
                }
            }
            if *task_title.read() != String::new(){
                h4 {
                    "{task_title.read()}"
                }
            }
            textarea {
                class: "task-heading",
                value: "{task_text}",
                tabindex: "0",
                oninput: move |event| task_text.set(event.value())
            }
            button {
                class: "task-delete",
                onclick: move |event| {
                    let task_list = tasks.read().clone();
                    async move{
                        delete(id).await;
                        /* TODO:: fix not deleting the right one visually */
                        tasks.set(
                            task_list.iter()
                                .cloned()
                                .filter(|i| i != &id)
                                .collect()
                        );
                    }
                },
                "X"
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
    rsx! {
        div{
            class: "header",
            h2 { {props.title} }
        }
    }
}

fn string_split(input: String) -> Vec<String> {
    if input.contains('~') {
        input.split('~').map(|s| s.to_string()).collect()
    } else {
        vec!["".to_string(), input]
    }
}
