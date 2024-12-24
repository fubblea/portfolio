use dioxus::prelude::*;

use crate::components::AllProjects;

const PROFILE_PHOTO: Asset = asset!("/assets/images/profile_photo.jpeg");

#[component]
pub fn Home() -> Element {
    rsx! {
        div { id: "landing",
            div { class: "column",
                h1 { "Hi, \n My name is Ajay" }
            }
            div { class: "column",
                img { src: PROFILE_PHOTO }
            }
        }

        div { id: "about-me",
            h1 { "About Me" }
            p { "Blah blah blah Blah" }
        }

        AllProjects {}
    }
}
