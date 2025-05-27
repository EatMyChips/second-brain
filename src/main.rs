mod components;
mod backend;

use dioxus::prelude::*;
use components::*;

const GLOBAL: Asset = asset!("assets/global.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/todo/weekly")]
    Weekly {},
}

// This is new in 0.6: it handles platform-specific launching
fn main() {
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: GLOBAL}
        Router::<Route> {}
    }
}