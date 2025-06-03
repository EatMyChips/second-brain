use std::rc::Rc;
use dioxus::prelude::*;
use crate::backend::*;
use crate::backend::props::Task;
use crate::components::todo::weekly::date_features::DailyTaskSwitcher;
use super::AppState;

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
    let mut tasks = use_signal(|| Vec::<i64>::new());

    // Date signals
    let selected_week = use_context::<AppState>().selected_week;
    let selected_day = use_context::<AppState>().selected_day;
    let edit_mode = use_context::<AppState>().edit_mode;

    // Props data
    let id = props.id.clone();

    // Get tasks if selected_week or selected_day updates
    let tasks_loading = use_resource(move || {
        let id: String = props.id.clone();

        // get current dates
        let day = if id == "todays-tasks" {
            Some(selected_day.read().format("%d/%m/%y").to_string())
        } else {
            None
        };
        let week: String = selected_week.read().format("%d/%m/%y").to_string();

        // The closure must return an async block
        async move {
            use_signal( async|| Task::get_all(id, week, day).await)
        }
    });

    rsx!{
        match &*tasks_loading.read_unchecked() {
            Some(mut tasks) => rsx! {
                document::Stylesheet { href: LISTS}

                div{
                    class: "element list",
                    id: id.clone(),
                    tabindex: "0",
                    onkeydown: move |event: Event<KeyboardData>| {
                        let id = id.clone();
                        async move {
                            let key = event.data.key();
                            if *edit_mode.read() {
                                if key == Key::Enter {
                                    let day = if id == "todays-tasks" {
                                        Some(selected_day.read().format("%d/%m/%y").to_string())
                                    } else {
                                        None
                                    };
                                    let week = Some(selected_week.read().format("%d/%m/%Y").to_string());

                                    tasks.read().push(Task::new(week, day, id).await);
                                }
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
                            TaskComp {id}
                        }
                    }
                }
            },
            None =>  rsx! { p { "Loading..." } }
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
            class: "task text",
            h3 {

            }

        }
        input {
            class: "task",
            value: "",
            tabindex: "0",
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