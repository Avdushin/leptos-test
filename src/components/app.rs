use leptos::*;
use leptos_meta::*;
use crate::routing::routes::Routing;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lep.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // {Routing()}
        <Routing />
    }
}