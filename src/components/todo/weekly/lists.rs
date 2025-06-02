use std::rc::Rc;
use dioxus::prelude::*;
use crate::backend::*;
use crate::backend::props::{Task, TaskInput};
use crate::components::todo::weekly::date_features::DailyTaskSwitcher;
use super::TimeState;

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
    let mut trigger_get = use_signal(|| 0);

    // Date signals
    let selected_week = use_context::<TimeState>().selected_week;
    let selected_day = use_context::<TimeState>().selected_day;

    // Props data
    let id = props.id.clone();

    // Element signals
    let input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let _ = use_resource(move || {
        let id = props.id.clone();
        let trigger = trigger_get.read();

        let mut day: Option<String> = None;

        if id == "todays-tasks" {
            day = Some(selected_day.read().format("%d/%m/%Y").to_string());
        }
        let week = selected_week.read().format("%d/%m/%Y").to_string();

        async move {
            tasks.set(Task::get_all(id, week, day).await);
        }
    });

    rsx!{
        document::Stylesheet { href: LISTS}

        div{
            class: "element list",
            id: id,
            tabindex: "0",
            onfocus: move |_| async move {
                if let Some(header) = input_element() {
                    let _ = header.set_focus(true).await;
                    new_task.set(String::new());
                }
            },
            ListHeader {
                title: props.title,
                id: id.clone(),
            }

            Tasks {
                new_task: new_task.clone(),
                input_element: input_element.clone(),
                tasks: tasks.clone(), // ✅ Pass signal
                id: id.clone(),
                trigger_get: trigger_get,
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
    pub trigger_get: Signal<i32>,
}

#[component]
pub fn Tasks(props: TasksProps) -> Element {
    // Task signals
    let mut tasks = props.tasks;
    let mut new_task = props.new_task;
    let mut update_task = use_signal(|| "".to_string());
    let mut trigger_get = props.trigger_get;

    // Element signals
    let mut input_element = props.input_element.clone();

    rsx!{
        div{
            class: "tasks",
            for task in tasks.read().clone() {
                div {
                    class: "task text",
                    h3 {
                        {task.title.clone()}
                    }
                    h3 {
                        "- {task.info.clone()}"
                    }
                }
                input {
                    class: "task",
                    value: format!("{}~{}", task.title, task.info),
                    tabindex: "0",
                    oninput: move |event| update_task.set(event.value()),
                    onkeydown: move |event: Event<KeyboardData>| {
                        let task = task.clone();
                        async move {
                            let key = event.data.key();

                            if key == Key::Delete {
                                &task.delete().await;
                                tasks.write().retain(|t| t.id != task.id);
                            }
                            if key == Key::Enter {
                                let mut task_data = string_split(update_task.read().clone());
                                task.update(task_data.remove(0), task_data.remove(0)).await;
                                let val = *trigger_get.read();
                                trigger_get.set(val + 1);
                                update_task.set(String::new());
                            }
                        }
                    },
                }
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
                            let mut task_data = string_split(new_task.read().clone());

                            let mut pass_data: TaskInput = TaskInput {
                                title: task_data.remove(0),
                                info: task_data.remove(0),
                                week: Some(use_context::<TimeState>().selected_week.read().format("%d/%m/%Y").to_string()),
                                day: None,
                                container_id: id.clone(),
                            };

                            if id == "todays-tasks" {
                                pass_data.day = Some(use_context::<TimeState>().selected_day.read().format("%d/%m/%Y").to_string())
                            }

                            tasks.write().push(Task::new(pass_data).await);
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
    id: String,
}

#[component]
fn ListHeader(props: ListHeaderProps) -> Element {
    rsx!{
        document::Stylesheet { href: HEADER}

        div{
            class: "header",
            h2 { {props.title} }

            if props.id == "todays-tasks"{
                DailyTaskSwitcher { }
            }
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