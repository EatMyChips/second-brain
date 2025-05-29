use std::rc::Rc;
use dioxus::prelude::*;
use crate::backend::*;
use crate::backend::props::{Task, TaskInput};

const LISTS: Asset = asset!("/assets/todo/weekly/lists.css");
const HEADER: Asset = asset!("/assets/todo/weekly/header.css");

#[derive(PartialEq, Props, Clone)]
pub struct ListProps {
    id: String,
    title: String,
}

#[component]
pub fn List(props: ListProps) -> Element {
    // Task signals
    let mut tasks = use_signal(|| Vec::<Task>::new());
    let mut new_task = use_signal(|| "".to_string());

    // Element signals
    let mut input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_effect({
        let mut tasks = tasks.clone();
        let id = props.id.clone();
        move || {
            let id = id.clone();
            spawn(async move {
                if let Ok(fetched) = get_tasks(id).await {
                    println!("Fetched tasks");
                    tasks.set(fetched);
                }
                else {
                    println!("Failed to fetch tasks");
                }
            });
        }
    });

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
            ListHeader { title: props.title }
            Tasks {
                new_task: new_task.clone(),
                input_element: input_element.clone(),
                tasks: tasks.clone(), // ✅ Pass signal
                id: props.id.clone(),
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct TasksProps {
    pub new_task: Signal<String>,
    pub input_element: Signal<Option<Rc<MountedData>>>,
    pub tasks: Signal<Vec<Task>>,
    pub id: String,
}

#[component]
pub fn Tasks(props: TasksProps) -> Element {
    // Task signals
    let mut tasks = props.tasks;
    let mut new_task = props.new_task.clone();

    // Element signals
    let mut input_element = props.input_element.clone();

    rsx!{
        div{
            class: "tasks",
            for task in tasks.read().clone() {
                h3 { class: "task", {task.info} }
            }

            input{
                class: "task",
                placeholder: "Enter new task",
                value: new_task,
                oninput: move |event| new_task.set(event.value()),
                onmounted: move |element| input_element.set(Some(element.data())),
                onkeydown: move |event: Event<KeyboardData>| {
                    let id = props.id.clone();
                    async move {
                        let key = event.data.key();

                        if key == Key::Enter {

                            let pass_data: TaskInput = TaskInput {
                                title: "".to_string(),
                                info: new_task.read().clone(),
                                week: None,
                                day: None,
                                container_id: 0,
                            };

                            tasks.write().push(post_tasks(id, pass_data).await.expect("Failed to post task").unwrap());
                            new_task.set(String::new());
                        }
                    }
                },
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