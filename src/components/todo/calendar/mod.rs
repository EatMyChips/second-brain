use dioxus::prelude::*;

const CALENDAR: Asset = asset!("/assets/todo/calendar.css");
const TODO: Asset = asset!("/assets/todo/todo.css");

#[component]
pub fn Calendar() -> Element {
    rsx!{
        document::Stylesheet { href: CALENDAR}
        document::Stylesheet { href: TODO}

        div {
            class: "page weekly",
            id: "calendar",
            div {
                class: "calendar",
            }
            div {
                class: "clock",
            }
        }
    }
}