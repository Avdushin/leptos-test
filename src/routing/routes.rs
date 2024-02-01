use leptos::*;
use leptos_router::*;

use crate::components::hp::HomePage;
use crate::components::nf::NotFound;

#[component]
pub fn Routing() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
