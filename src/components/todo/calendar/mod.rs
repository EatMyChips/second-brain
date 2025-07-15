use std::cmp::PartialEq;
use crate::components::todo::{AppState};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Timelike, Weekday};
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use std::rc::Rc;

const CALENDAR: Asset = asset!("/assets/todo/calendar.css");
const TODO: Asset = asset!("/assets/todo/todo.css");

#[derive(Clone)]
pub struct Month {
    pub year: i32,
    pub month: u32
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
    // get all date props
    let mut selected_month = use_context::<AppState>().selected_month;
    let mut selected_week = use_context::<AppState>().selected_week;
    let mut selected_day = use_context::<AppState>().selected_day;
    let current_month = use_context::<AppState>().current_month;
    let current_week = use_context::<AppState>().current_week;
    let current_day = use_context::<AppState>().current_day;

    // Instantiate display data
    let mut active_month_days= use_signal(|| 0);
    let mut first_day_count= use_signal(|| 0);
    let mut month_name = use_signal(|| String::new());
    let mut left_over = use_signal(|| 0);
    let mut prev_month_days = use_signal(|| 0);
    let mut row_count = use_signal(|| 0);

    // Set display data on selected month change
    let _ = use_resource( move || {
        // Reactive with selected_month
        let selected_month = selected_month.read().clone();
        let current_month = current_month.read().clone();

        active_month_days.set(days_in_month(selected_month.clone()));

        // Selected day
        if (current_month.month == selected_month.month) && (current_month.year == selected_month.year)  {
            selected_day.set(*current_day.read());
        }
        else {
            selected_day.set(NaiveDate::from_ymd_opt(selected_month.year, selected_month.month, 1).unwrap());
        }

        // Working out day counts
        first_day_count.set(first_day_of_month(selected_month.clone()));
        let total_days = *active_month_days.read() + *first_day_count.read();
        let remainder = (total_days - 1) % 7;
        let extra_days = if remainder == 0 { 0 } else { 7 - remainder };
        left_over.set(if extra_days == 0 { 1 } else {extra_days + 1});
        let total_cells = total_days + extra_days;
        row_count.set(total_cells / 7);

        // Previous month
        let prev_month = Month { year: selected_month.year, month: selected_month.month - 1 };
        prev_month_days.set(days_in_month(prev_month));
        async move {}
    });

    // Set display data on selected day change
    let _ = use_resource( move || {
        let selected_day = selected_day.read();
        month_name.set(selected_day.format("%B").to_string());
        selected_week.set({
            let date = *selected_day;
            let weekday: u32 = date.weekday().num_days_from_monday();
            date - Duration::days(weekday.into())
        });
        async move {}
    });

    rsx! {
        div {
            class: "calendar",
            div {
                class: "month",
                h3 {
                    {month_name.read().clone()}
                }
                button {
                    onclick: move |_|{
                        let month = selected_month.read().clone();
                        selected_month.set(previous_month(month));
                    },
                    "<-"
                }
                button {
                    onclick: move |_|{
                        let month = selected_month.read().clone();
                        selected_month.set(next_month(month));
                    },
                    "->"
                }
            }
            div {
                class: "days",
                style: "grid-template-rows: repeat({row_count}, 1fr);",
                for i in (prev_month_days - *first_day_count.read() + 2)..(prev_month_days + 1) {
                    Day { number: i, inactive: true, selected: false }
                }
                for i in 1..(*active_month_days.read() + 1) {
                    Day { number: i, inactive: false, selected: selected_day.read().day() == i }
                }
                for i in 1..*left_over.read(){
                    Day { number: i, inactive: true, selected: false }
                }
            }
        }
    }
}

#[component]
fn Day(number: u32, inactive: bool, selected: bool) -> Element {
    let class_name = if inactive { "day inactive" } else if selected { "day selected" } else { "day" };
    let mut selected_day = use_context::<AppState>().selected_day;
    let month = use_context::<AppState>().selected_month;

    rsx! {
        div {
            class: "{class_name}",
            tabindex: 1,
            onfocus: move |_| {
                if !inactive {
                    selected_day.set(NaiveDate::from_ymd_opt(month.read().year, month.read().month, number).unwrap());
                }
            },
            h3 { "{number}" }
        }
    }
}

fn days_in_month(month: Month) -> u32 {
    let next_date = next_month(month);

    let first_of_next_month = NaiveDate::from_ymd_opt(next_date.year, next_date.month, 1).unwrap();
    let last_day = first_of_next_month.pred_opt().unwrap();

    last_day.day()
}

fn next_month(month: Month) -> Month{
    match month.month {
        12 => {
            Month { year: month.year + 1, month: 1 }
        }
        _ => {
            Month { year: month.year, month: month.month + 1 }
        }
    }
}

fn previous_month(mut month: Month) -> Month{
    match month.month {
        1 => {
            Month { year: month.year - 1, month: 12 }
        }
        _ => {
            Month { year: month.year, month: month.month - 1 }
        }
    }
}

fn first_day_of_month(month: Month) -> u32 {
    let first_day = Local.ymd(month.year, month.month, 1);
    weekday_to_number(first_day.weekday())
}

fn weekday_to_number(day: Weekday) -> u32 {
    match day {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    }
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
