use std::rc::Rc;
use dioxus::prelude::*;

const CALENDAR: Asset = asset!("/assets/todo/calendar.css");
const TODO: Asset = asset!("/assets/todo/todo.css");

#[component]
pub fn Calendar(calendar: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx!{
        document::Stylesheet { href: CALENDAR}
        document::Stylesheet { href: TODO}

        div {
            class: "page daily",
            id: "calendar",
            onmounted: move |element| async move {
                calendar.set(Some(element.data))
            },
            div {
                class: "calendar",
            }
            div {
                class: "clock",
            }
        }
    }
}