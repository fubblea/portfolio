use leptos::*;

/// 404 Not Found Page
#[component]
#[expect(non_snake_case)]
pub fn NotFound() -> impl IntoView {
    view! { <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>  }
}
