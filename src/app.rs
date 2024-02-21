#![allow(
    clippy::empty_structs_with_brackets,
    clippy::infinite_loop,
    clippy::module_name_repetitions
)] // reason = "Leptos components do not appropriately allow or expect lints"
use crate::components::Leaderboard;
use leptos::{component, tracing, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/uknd.css"/>

        // sets the document title
        <Title text="UKND LB"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link
            href="https://fonts.googleapis.com/css2?family=Inter:wght@300;500;700&display=swap"
            rel="stylesheet"
        />
        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/lb" view=Leaderboard/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1 class="title">"ULTRAKILL NO DAMAGE Leaderboards"</h1>
        <Leaderboard/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        use actix_web::http::StatusCode;
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = leptos::expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(StatusCode::NOT_FOUND);
    };

    view! { <h1>"Not Found"</h1> }
}
