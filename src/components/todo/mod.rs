pub(crate) mod weekly;

use dioxus::prelude::*;

const TODO: Asset = asset!("assets/todo/todo.css");

#[component]
pub fn Header() -> Element{
    rsx!{
        document::Stylesheet { href: TODO}
        div {
            class: "nav-bar",
             button {
                class: "nav-button",
                "Weekly Tasks"
            }
             button {
                class: "nav-button",
                "Tasks"
            }
             button {
                class: "nav-button",
                id: "rewards",
                "Rewards"
            }
        }
    }
}