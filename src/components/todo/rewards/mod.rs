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
                "Days"
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

const DAYS: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

#[component]
fn CheckBox(id: i32) -> Element {
    // Switch from Vec<i32> to a fixed-size boolean array for clarity and type safety
    let mut checked = use_signal(|| [false; 7]);
    rsx! {
        div {
            class: "challenge",
            h4 {
                class: "title",
                "{id}. Take out bins"
            }
            div {
                class: "check-obj",
                // Loop over days to avoid duplicate IDs and boilerplate
                for (idx, day) in DAYS.iter().enumerate() {
                    input {
                        class: "check",
                        id: "{id}-{day}",
                        r#type: "checkbox",
                        // Controlled input to keep UI and state in sync
                        checked: checked.read()[idx],
                        onchange: move |_| {
                            let mut w = checked.write();
                            w[idx] = !w[idx];
                        }
                    }
                }
            }
        }
    }
}
