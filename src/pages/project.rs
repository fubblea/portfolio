use leptos::*;

#[component]
#[expect(non_snake_case)]
pub fn Project() -> impl IntoView {
    view! { <h1>"This is a project page" <br/> "Hello there"</h1>  }
}

#[component]
#[expect(non_snake_case)]
pub fn ProjectOverview() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
        <h1 class="p-6 text-4xl">"This is an overview of all my projects"</h1>

        </div>
    }
}
