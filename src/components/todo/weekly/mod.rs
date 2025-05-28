mod lists;

use dioxus::prelude::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use lists::*;

const WEEKLY: Asset = asset!("/assets/weekly.css");

#[component]
pub fn Weekly() -> Element {
    rsx!{
        document::Stylesheet { href: WEEKLY}

        super::Header {}
        div{
            class: "weekly-lists",
            TodaysTasks {}
            CheckBoxes {}
            div { class: "break" }
            University {}
            Personal {}
            Life {}
        }

    }
}

#[component]
fn TodaysTasks() -> Element {
    rsx!{
        List {
            id: String::from("todays-tasks"),
            title: String::from("Today's Tasks"),
        }
    }
}

#[component]
fn CheckBoxes() -> Element {
    rsx!{
        div{
            class: "element",
            tabindex: "0",
        }
    }
}

#[component]
fn University() -> Element {
    rsx!{
        List {
            id: String::from(""),
            title: String::from("University"),
        }
    }
}

#[component]
fn Personal() -> Element {
    rsx!{
        List {
            id: String::from(""),
            title: String::from("Personal"),
        }
    }
}

#[component]
fn Life() -> Element {
    rsx!{
        List {
            id: String::from(""),
            title: String::from("Life"),
        }
    }
}