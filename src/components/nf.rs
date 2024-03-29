use leptos::*;
use leptos_meta::*;

/// 404 - Not Found
#[component]
pub fn NotFound() -> impl IntoView {
    // provide_meta_context();

    
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="Page Not Found" />
        <h1>"Not Found"</h1>
    }
}