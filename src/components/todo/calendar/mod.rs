use std::rc::Rc;
use chrono::{Local, Timelike};
use dioxus::prelude::*;
use crate::components::todo::AppState;
use gloo_timers::future::IntervalStream;
use futures_util::stream::StreamExt;


const CALENDAR: Asset = asset!("/assets/todo/calendar.css");
const TODO: Asset = asset!("/assets/todo/todo.css");

#[component]
pub fn Calendar(calendar: Signal<Option<Rc<MountedData>>>) -> Element {
    let current_time = use_signal(|| Local::now().time());

    // updates time every 10 seconds
    use_coroutine(move |_: UnboundedReceiver<()>| {
        to_owned![current_time];
        async move {
            let mut ticks = IntervalStream::new(10000); // every 1000ms
            while ticks.next().await.is_some() {
                current_time.set(Local::now().time());
            }
        }
    });

    let mut time = current_time.read();

    // Extract the digits as characters
    let hour = time.hour();
    let minute = time.minute();

    // Format to 2-digit strings
    let hour_str = format!("{:02}", hour);
    let minute_str = format!("{:02}", minute);

    // Combine to one string to easily access by index
    let time_digits: Vec<char> = format!("{}{}", hour_str, minute_str).chars().collect();

    rsx!{
        document::Stylesheet { href: CALENDAR}
        document::Stylesheet { href: TODO}

        div {
            class: "page daily",
            id: "calendar",
            onmounted: move |element| async move {
                calendar.set(Some(element.data))
            },
            CalendarObj {}
            div {
                class: "clock",
                h1 {
                    class: "clock-digits",
                    "{time_digits[0]}"
                }
                h1 {
                    class: "clock-digits",
                    "{time_digits[1]}"
                }
                h1 {
                    class: "clock-break",
                    ":"
                }
                h1 {
                    class: "clock-digits",
                    "{time_digits[2]}"
                }
                h1 {
                    class: "clock-digits",
                    "{time_digits[3]}"
                }
            }
        }
    }
}

#[component]
fn CalendarObj() -> Element{
    rsx!{
        div {
            class: "calendar",
             h3 {
                class: "month",
                "June"
            }
            div {
                class: "days",
                 for i in 0..35 {
                    div {
                        class: "day",
                        h3 {
                            "1"
                        }
                    }
                }
            }
        }
    }
}