use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        h1 { "Requested URL not found: {segments:?}" }

        Link { to: Route::Home {}, "Go home!" }
    }
}
