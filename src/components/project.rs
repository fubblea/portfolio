use dioxus::prelude::*;

#[component]
pub fn Project(id: usize) -> Element {
    rsx! {
        h1 { "Project {id}" }
    }
}
