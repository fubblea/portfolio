use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { id: "header",
            h1 { "Ajay B. Anand" }
            div { id: "navbar",
                Link { to: Route::Home {}, "Home" }
                Link { to: "/#about-me", "About Me" }
                Link { to: "/#projects", "Projects" }
            }
        }

        Outlet::<Route> {}
    }
}
