use dioxus::prelude::*;

const CHECKS: Asset = asset!("assets/todo/checks.css");

#[component]
pub fn Checks() -> Element {
    let checks = vec! {1,2,3};
    /*TODO:: Fix styling of days */
    rsx! {
        document::Stylesheet { href: CHECKS }
        div {
            class: "checks",
            WeekDays {}
            for i in checks {
                CheckBox { id: i }
            }
        }
    }
}

#[component]
fn WeekDays() -> Element {
    rsx! {
        div {
            class: "challenge",
            h4 {
                class: "title",

            }
            div {
                class: "check-obj",
                h4 {
                    "M"
                }
                h4 {
                    "T"
                }
                h4 {
                    "W"
                }
                h4 {
                    "T"
                }
                h4 {
                    "F"
                }
                h4 {
                    "S"
                }
                h4 {
                    "S"
                }
            }
        }
    }
}

#[component]
fn CheckBox( id: i32) -> Element {
    let mut checked = use_signal(|| vec![0,0,0,0,0,0,0]);
    rsx! {
        div {
            class: "challenge",
            h4 {
                class: "title",
                "{id}.Take out bins"
            }
            div {
                class: "check-obj",
                input {
                    class: "check",
                    id: "mon",
                    type: "checkbox",
                    onchange: move |evt| {
                        checked.write()[0] = (evt.value() == "true") as i32;
                    }
                }
                input {
                    class: "check",
                    id: "tue",
                    type: "checkbox",
                    onchange: move |evt| {
                        checked.write()[1] = (evt.value() == "true") as i32;
                    }
                }
                input {
                    class: "check",
                    id: "wed",
                    type: "checkbox",
                    onchange: move |evt| {
                        checked.write()[2] = (evt.value() == "true") as i32;
                    }
                }
                input {
                    class: "check",
                    id: "thu",
                    type: "checkbox",
                    onchange: move |evt| {
                        checked.write()[3] = (evt.value() == "true") as i32;
                    }
                }
                input {
                    class: "check",
                    id: "fri",
                    type: "checkbox",
                    onchange: move |evt| {
                        checked.write()[4] = (evt.value() == "true") as i32;
                    }
                }
                input {
                    class: "check",
                    id: "sat",
                    type: "checkbox",
                    onchange: move |evt| {
                        checked.write()[5] = (evt.value() == "true") as i32;
                    }
                }
                input {
                    class: "check",
                    id: "sun",
                    type: "checkbox",
                    onchange: move |evt| {
                        checked.write()[6] = (evt.value() == "true") as i32;
                    }
                }
            }
        }
    }
}
