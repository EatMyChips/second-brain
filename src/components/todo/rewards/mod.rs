use dioxus::prelude::*;

const REWARDS: Asset = asset!("assets/todo/rewards.css");

#[component]
pub fn Checks() -> Element {
    let checks = vec! {1,2,3};

    rsx! {
        document::Stylesheet { href: REWARDS }
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

    }
}
