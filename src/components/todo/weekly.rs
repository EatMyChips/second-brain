use dioxus::prelude::*;
use web_sys::HtmlElement;
use std::rc::Rc;

const WEEKLY: Asset = asset!("/assets/weekly.css");

#[component]
pub fn Weekly() -> Element {
    rsx!{
        document::Stylesheet { href: WEEKLY}

        super::Header {}

        TodaysTasks {}
        div{
            class: "weekly-lists",
            University {}
            Personal {}
            Life {}
        }

    }
}



fn TaskList() -> Element {
    let tasks:Vec<&str> = vec!["task 1", "task 2", "task 3"];

    let mut new_task = use_signal(|| "".to_string());
    let mut input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    rsx!{
        div{
            class: "list",
            id: "todays-tasks",
            tabindex: "0",
            onfocus: move |_| async move {
                    if let Some(header) = input_element() {
                        let _ = header.set_focus(true).await;
                        new_task.set(String::new());
                    }
                },
            div{
                class: "tasks",
                for task in tasks {
                    h3{
                        class: "task",
                        {task}
                    }
                }

                input{
                    class: "task",
                    placeholder: "Enter new task",
                    value: "{new_task}",
                    oninput: move |event| new_task.set(event.value()),
                    onmounted: move |element| input_element.set(Some(element.data())),
                }
            }

        }
    }
}

#[component]
fn TodaysTasks() -> Element {
    rsx!{
        TaskList {}
    }
}

#[component]
fn University() -> Element {
    rsx!{
        TaskList {}
    }
}

#[component]
fn Personal() -> Element {
    rsx!{
        TaskList {}
    }
}

#[component]
fn Life() -> Element {
    rsx!{
        TaskList {}
    }
}