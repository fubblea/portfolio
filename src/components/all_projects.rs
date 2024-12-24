use dioxus::prelude::*;

#[component]
pub fn AllProjects() -> Element {
    rsx! {
        div { id: "projects",
            h1 { "Projects" }
            p { "Look at all the cool work I did" }
        }
    }
}
