use leptos::*;
use leptos_router::use_params_map;

/// 404 Not Found Page
#[component]
#[expect(non_snake_case)]
pub fn NotFound() -> impl IntoView {
    let params = use_params_map();
    let q = move || {
        params.with(|params| {
            params
                .get("q")
                .cloned()
                .unwrap_or("Can't get url".to_string())
        })
    };

    view! { <h1>"Uh oh!" <br/> "Page not found: " {q}</h1>  }
}
