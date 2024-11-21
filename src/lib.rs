use components::nav_bar::NavBar;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::not_found::NotFound;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::project::Project;

/// An app router which renders the homepage and handles 404's
#[component]
#[expect(non_snake_case)]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    logging::log!("lib::App - Starting app");

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>
        <Stylesheet id="leptos" href="/style/output.css"/>

        // sets the document title
        <Title text="Ajay B. Anand"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <NavBar/>
            <main>
                <Routes>
                    <Route path="/portfolio/" view=Home/>
                    <Route path="/portfolio/project" view=Project/>
                    <Route path="*" view=Home/>
                </Routes>
            </main>
        </Router>
    }
}
