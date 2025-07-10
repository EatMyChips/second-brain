use crate::components::todo::{AppState, ScrollState};
use dioxus::prelude::*;

const HEADER: Asset = asset!("assets/todo/header.css");

#[component]
pub fn Header() -> Element {
    let scroll_state = use_context::<AppState>().scroll_state;

    let mut text1 = use_signal(|| String::new());
    let mut text2 = use_signal(|| String::new());
    let mut text3 = use_signal(|| String::new());

    let _ = use_resource(move || {
        match *scroll_state.read() {
            ScrollState::Rewards => {
                text1.set(String::from(""));
                text2.set(String::from("Rewards"));
                text3.set(String::from("Daily"));
            }
            ScrollState::Daily => {
                text1.set(String::from("Rewards"));
                text2.set(String::from("Daily"));
                text3.set(String::from("Weekly"));
            }
            ScrollState::Weekly => {
                text1.set(String::from("Daily"));
                text2.set(String::from("Weekly"));
                text3.set(String::from(""));
            }
        }
        async move {}
    });

    rsx! {
        document::Stylesheet { href: HEADER}
        div {
            class: "nav-bar",
             button {
                class: "nav-button",
                "{text1}"
            }
             button {
                class: "nav-button selected",
                "{text2}"
            }
             button {
                class: "nav-button",
                "{text3}"
            }
        }
    }
}
