use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/uknd.css"/>

        // sets the document title
        <Title text="UKPND LB"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link
            href="https://fonts.googleapis.com/css2?family=Inter:wght@300;500&display=swap"
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
#[allow(non_snake_case)]
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1 class="title">"ULTRAKILL P-Rank NO DAMAGE Leaderboard"</h1>
        <Leaderboard/>
    }
}

use crate::gamedata::{self, Run};

/// Contains both the data and the view management of it
#[allow(non_snake_case)]
#[component]
fn Leaderboard() -> impl IntoView {
    let gamedata = {
        let mut gamedata = gamedata::load_runs();
        gamedata.sort_by(|a, b| {
            a.igt_ms
                .cmp(&b.igt_ms)
                .then_with(|| a.submission_date.cmp(&b.submission_date))
        });
        gamedata
    };
    let rows: Vec<_> = gamedata
        .iter()
        .enumerate()
        .map(|(idx, run)| {
            let Run {
                runner,
                igt_ms,
                submission_date,
                difficulty,
                patch_release_date,
            } = run;
            let igt_ms = format!(
                "{:02}:{:02}:{:03}",
                igt_ms / 1000 / 60,
                igt_ms / 1000 % 60,
                igt_ms % 1000
            );
            let submission_date = {
                use std::time::{Duration, SystemTime};
                let seconds = (SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards")
                    - Duration::from_secs(*submission_date))
                .as_secs();

                let minutes = seconds / 60;
                let hours = minutes / 60;
                let days = hours / 24;
                let months = days / 30;
                let years = days / 365;
                format!(
                    "{} ago",
                    match (years, months, days, hours, minutes, seconds) {
                        (y, _, _, _, _, _) if y > 0 =>
                            format!("{} year{}", y, if y == 1 { "" } else { "s" }),
                        (_, m, _, _, _, _) if m > 1 =>
                            format!("{} month{}", m, if m == 1 { "" } else { "s" }),
                        (_, _, d, _, _, _) if d > 1 =>
                            format!("{} day{}", d, if d == 1 { "" } else { "s" }),
                        (_, _, _, h, _, _) if h > 1 =>
                            format!("{} hour{}", h, if h == 1 { "" } else { "s" }),
                        (_, _, _, _, m, _) if m > 1 =>
                            format!("{} minute{}", m, if m == 1 { "" } else { "s" }),
                        (_, _, _, _, _, s) =>
                            format!("{} second{}", s, if s == 1 { "" } else { "s" }),
                    }
                )
            };
            let difficulty = difficulty.to_string();
            let patch_release_date = patch_release_date.to_string();
            view! {
                <tr>
                    <td>{idx + 1}</td>
                    <td>
                        <a href="https://www.speedrun.com/users/{runner}">{runner}</a>
                    </td>
                    <td>{igt_ms}</td>
                    <td>{submission_date}</td>
                    <td>{difficulty}</td>
                    <td>{patch_release_date}</td>
                </tr>
            }
        })
        .collect();
    view! {
        <div style="display: flex; justify-content: center;">
            <table class="grid">
                <thead>
                    <tr>
                        <th>"#"</th>
                        <th>"Runner"</th>
                        <th>"IGT"</th>
                        <th>"Date"</th>
                        <th>"Difficulty"</th>
                        <th>"Patch"</th>
                    </tr>
                </thead>
                <tbody>{rows}</tbody>
            </table>
        </div>
    }
}

/// 404 - Not Found
#[allow(non_snake_case)]
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
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
