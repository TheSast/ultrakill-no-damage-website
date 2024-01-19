use crate::gamedata::{self, Category, Run};
use leptos::{
    component, create_signal, leptos_dom::tracing, view, IntoAttribute, IntoView, SignalGet,
    SignalUpdate,
};

#[component]
pub fn Leaderboard() -> impl IntoView {
    let runs = {
        let mut runs = gamedata::load_runs();
        runs.sort_by(|a, b| {
            a.igt_ms
                .cmp(&b.igt_ms)
                .then_with(|| b.difficulty.cmp(&a.difficulty))
                .then_with(|| b.patch_release_date.cmp(&a.patch_release_date))
                .then_with(|| a.submission_date.cmp(&b.submission_date))
                .then_with(|| a.runner.cmp(&b.runner))
        });
        runs.dedup_by(|a, b| a.runner == b.runner);
        runs
        // [100%] 1. Runs should be ordered by length, favoring harder then up-to-date then older runs in a tie
        // [100%] 2. Runs should contain their category? and P% runs auto-count as any% ones
        // [100%] 2. Runs should contain their related level
        // [020%] 3. Non-PB runs by the same runner should be filtered out to seperate class
    };
    let (sr, sw) = create_signal(Category::P);
    let change_category = move |_| {
        sw.update(|c| {
            *c = match *c {
                Category::P => Category::Any,
                Category::Any => Category::NoMo,
                Category::NoMo => Category::P,
            }
        });
    };
    view! {
        <div style="display: grid; justify-content: center;">
            <button on:click=change_category>{move || sr.get().to_string()}</button>
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
                <tbody>{move || { runs_into_view(&runs, &sr.get()) }}</tbody>
            </table>
        </div>
    }
}

#[expect(
    clippy::pattern_type_mismatch,
    reason = "same-name variable deconstruction to references is not a type mismatch"
)]
fn runs_into_view(runs: &[Run], category: &Category) -> impl IntoView {
    runs.iter()
        .filter(|r| {
            r.category == *category || *category == Category::Any && r.category == Category::P
        })
        .enumerate()
        .map(|(idx, run)| {
            let Run {
                runner,
                // level: _,
                igt_ms,
                // category: _,
                submission_date,
                difficulty,
                patch_release_date,
                proof,
                ..
            } = run;
            let igt_ms = format!(
                "{:02}:{:02}:{:03}",
                igt_ms / 1000 / 60,
                igt_ms / 1000 % 60,
                igt_ms % 1000
            );
            let submission_date = {
                // use core::time::{Duration, SystemTime};
                // let seconds = (SystemTime::now()
                //     .duration_since(SystemTime::UNIX_EPOCH)
                //     .expect("Time went backwards")
                //     - Duration::from_secs(*submission_date))
                // .as_secs();
                // std::time is not implemented for wasm
                let seconds = *submission_date;

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
                        (_, m, _, _, _, _) if m > 0 =>
                            format!("{} month{}", m, if m == 1 { "" } else { "s" }),
                        (_, _, d, _, _, _) if d > 0 =>
                            format!("{} day{}", d, if d == 1 { "" } else { "s" }),
                        (_, _, _, h, _, _) if h > 0 =>
                            format!("{} hour{}", h, if h == 1 { "" } else { "s" }),
                        (_, _, _, _, m, _) if m > 0 =>
                            format!("{} minute{}", m, if m == 1 { "" } else { "s" }),
                        (_, _, _, _, _, s) =>
                            format!("{} second{}", s, if s == 1 { "" } else { "s" }),
                    }
                )
            };
            let difficulty = difficulty.to_string();
            let patch_release_date = patch_release_date.to_string();
            // INFO: need to call `into_attribute` manually to silence `unused_import` warning
            let proof = proof.to_string().into_attribute();
            // let proof = proof.to_string();
            let runner_link = format!("https://www.speedrun.com/users/{runner}");
            view! {
                <tr>
                    <td>{idx + 1}</td>
                    <td>
                        <a href=runner_link>{runner}</a>
                    </td>
                    <td>
                        <a href=proof>{igt_ms}</a>
                    </td>
                    <td>{submission_date}</td>
                    <td>{difficulty}</td>
                    <td>{patch_release_date}</td>
                </tr>
            }
        })
        .collect::<Vec<_>>()
}
