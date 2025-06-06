mod components;
mod backend;

use dioxus::prelude::*;
use components::*;
pub use backend::*;

const GLOBAL: Asset = asset!("assets/global.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/todo")]
    Tasks {},
}

// This is new in 0.6: it handles platform-specific launching
fn main() {
    // Enable panic messages in the browser console
    console_error_panic_hook::set_once();

    // Redirect log output to browser console (e.g., `log::info!`)
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: GLOBAL}
        Router::<Route> {}
    }
}