use dioxus::prelude::*;

const CHECKS: Asset = asset!("assets/todo/checks.css");

#[component]
pub fn Checks() -> Element {
    let checks = vec! {1,2,3};

    rsx! {
        document::Stylesheet { href: CHECKS }
        div {
            class: "checks",
            for i in checks {
                CheckBox {}
            }
        }
    }
}

#[component]
pub fn CheckBox() -> Element {
    rsx! {
        div {
            class: "challenge",
            h3 {
                class: "title",
                "Task 1"
            }
            input {
                class: "check",
                id: "mon",
                type: "checkbox",
            }
            input {
                class: "check",
                id: "tue",
                type: "checkbox",
            }
            input {
                class: "check",
                id: "wed",
                type: "checkbox",
            }
            input {
                class: "check",
                id: "thu",
                type: "checkbox",
            }
            input {
                class: "check",
                id: "fri",
                type: "checkbox",
            }
            input {
                class: "check",
                id: "sat",
                type: "checkbox",
            }
            input {
                class: "check",
                id: "sun",
                type: "checkbox",
            }
        }
    }
}
