use dioxus::prelude::*;

const HEADER: Asset = asset!("assets/todo/header.css");

#[component]
pub fn Header() -> Element{
    rsx!{
        document::Stylesheet { href: HEADER}
        div {
            class: "nav-bar",
             button {
                class: "nav-button",
                "Rewards"
            }
             button {
                class: "nav-button selected",
                "Daily"
            }
             button {
                class: "nav-button",
                "Weekly"
            }
        }
    }
}