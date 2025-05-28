use std::rc::Rc;
use dioxus::prelude::*;
use web_sys::HtmlElement;
use crate::backend::*;

const LISTS: Asset = asset!("/assets/todo/weekly/lists.css");
const HEADER: Asset = asset!("/assets/todo/weekly/header.css");

#[derive(PartialEq, Props, Clone)]
pub struct ListProps {
    id: String,
    title: String,
}

#[component]
pub fn List(props: ListProps) -> Element {
    let mut new_task = use_signal(|| "".to_string());
    let mut input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut tasks: Signal<Vec<String>> = use_signal(|| vec![]);

    rsx!{
        document::Stylesheet { href: LISTS}

        div{
            class: "element list",
            id: props.id,
            tabindex: "0",
            onfocus: move |_| async move {
                if let Some(header) = input_element() {
                    let _ = header.set_focus(true).await;
                    new_task.set(String::new());
                }
            },
            onkeydown: move |event: Event<KeyboardData>| async move {
                let key = event.data.key();

                if key == Key::Enter {
                    post_tasks(new_task.read().clone(), String::from("penis"), Some(String::from("10/01/2025")), Some(String::from("10/01/2025")), 1).await.expect("Failed to post task");
                    tasks.write().push(new_task.read().clone());
                    new_task.set(String::new());
                }
            },
            ListHeader { title: props.title }
            Tasks {
                new_task: new_task.clone(),
                input_element: input_element.clone(),
                tasks: tasks.clone(), // ✅ Pass signal
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct TasksProps {
    pub new_task: Signal<String>,
    pub input_element: Signal<Option<Rc<MountedData>>>,
    pub tasks: Signal<Vec<String>>,
}

#[component]
pub fn Tasks(props: TasksProps) -> Element {
    let tasks = props.tasks;
    let mut input_element = props.input_element.clone();
    let mut new_task = props.new_task.clone();

    rsx!{
        div{
            class: "tasks",
            for task in tasks() {
                h3 { class: "task", {task} }
            }

            input{
                class: "task",
                placeholder: "Enter new task",
                value: new_task,
                oninput: move |event| new_task.set(event.value()),
                onmounted: move |element| input_element.set(Some(element.data())),
            },
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct ListHeaderProps {
    title: String,
}

#[component]
fn ListHeader(props: ListHeaderProps) -> Element {
    rsx!{
        document::Stylesheet { href: HEADER}

        div{
            class: "header",
            h2 { {props.title} }
        }
    }
}