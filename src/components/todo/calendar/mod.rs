use crate::components::todo::AppState;
use chrono::{Datelike, Local, NaiveDate, Timelike};
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use std::rc::Rc;

const CALENDAR: Asset = asset!("/assets/todo/calendar.css");
const TODO: Asset = asset!("/assets/todo/todo.css");

enum months {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

#[component]
pub fn Calendar(calendar: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        document::Stylesheet { href: CALENDAR}
        document::Stylesheet { href: TODO}

        div {
            class: "page daily",
            id: "calendar",
            onmounted: move |element| async move {
                calendar.set(Some(element.data))
            },
            CalendarObj {}
            ClockObj {}
        }
    }
}

#[component]
fn CalendarObj() -> Element {
    let ThisMonthDays = use_signal(|| days_in_month(Local::now().year(), Local::now().month()));

    rsx! {
        div {
            class: "calendar",
             h3 {
                class: "month",
                "June"
            }
            div {
                class: "days",
                for i in 1..4 {
                    div {
                        class: "day",
                        h3 {
                            "{i}"
                        }
                    }
                }
                for i in 1..(*ThisMonthDays.read() + 1) {
                    div {
                        class: "day",
                        h3 {
                            "{i}"
                        }
                    }
                }
                for i in 1..2{
                    div {
                        class: "day",
                        h3 {
                            "{i}"
                        }
                    }
                }
            }
        }
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    // First day of the next month
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    // Get the last day of the current month by subtracting 1 day from the 1st of the next month
    let first_of_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    let last_day = first_of_next_month.pred_opt().unwrap();

    last_day.day()
}

#[component]
fn ClockObj() -> Element {
    let current_time = use_signal(|| Local::now().time());

    // updates time every 10 seconds
    use_coroutine(move |_: UnboundedReceiver<()>| {
        to_owned![current_time];
        async move {
            let mut ticks = IntervalStream::new(10000);
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

    rsx! {
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
