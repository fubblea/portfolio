use dioxus::{logger::tracing::info, prelude::*};

use components::{Navbar, Project};
use views::{Home, NotFound};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/portfolio")]
    Home {},

    #[route("/portfolio/project/:id")]
    Project {id:usize},

    //  if the current location doesn't match any of the above routes, render the NotFound component
    #[route("/portfolio/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    info!("Starting app...");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
