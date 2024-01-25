use crate::gamedata::{self, Category, Run};
use leptos::{
    component, create_signal, event_target_value,
    html::{Option_, Tr},
    leptos_dom::tracing,
    view, with, HtmlElement, IntoAttribute, IntoClass, IntoView, SignalGet, SignalSet, SignalWith,
};

// TODO:
// 1. Add options to filter by:
// - run obsoletion
// - difficulty
// - date
// - patch
// 2. Block out component if run data is missing
#[component]
pub fn Leaderboard() -> impl IntoView {
    let runs = {
        let mut runs = gamedata::load_runs();
        runs.sort_by(|a, b| {
            a.level
                .cmp(&b.level)
                .then(a.igt_ms.cmp(&b.igt_ms))
                .then(b.difficulty.cmp(&a.difficulty))
                .then(b.patch_release_date.cmp(&a.patch_release_date))
                .then(a.submission_date.cmp(&b.submission_date))
                .then(a.runner.cmp(&b.runner))
                .then_with(|| panic!("Duplicate run or mathematically impossible tie"))
        });
        runs
    };
    let (category_r, category_w) = create_signal(Category::Any);
    let (level_r, level_w) = create_signal(
        runs.first()
            .map_or_else(|| todo!("run data presence handling"), |r| r.level.clone()),
    );
    // INFO: need to call `into_class` manually to silence `unused_import` warning
    let _silencer = true.into_class();
    view! {
        <div class="leaderboard">
            <div class="controls">
                <div>
                    <div>
                        <select on:change=move |ev| {
                            level_w.set(event_target_value(&ev));
                        }>

                            {
                                let runs = runs.clone();
                                move || levels_into_options(&runs, &level_r.get())
                            }

                        </select>
                    </div>
                    <div>
                        <button
                            on:click=move |_| category_w.set(Category::Any)
                            class:selected=move || category_r.with(|c| *c == Category::Any)
                        >
                            "Any%"
                        </button>
                        <button
                            on:click=move |_| category_w.set(Category::P)
                            class:selected=move || category_r.with(|c| *c == Category::P)
                        >
                            "P Rank"
                        </button>
                        <button
                            on:click=move |_| category_w.set(Category::NoMo)
                            class:selected=move || category_r.with(|c| *c == Category::NoMo)
                        >
                            "NoMo"
                        </button>
                    </div>
                </div>
            </div>
            <table>
                <thead>
                    <tr>
                        <th>"#"</th>
                        <th>"Player"</th>
                        <th>"IGT"</th>
                        <th>"Date"</th>
                        <th>"Difficulty"</th>
                        <th>"Patch"</th>
                    </tr>
                </thead>
                <tbody>

                    {move || runs_into_trs(
                        &runs
                            .iter()
                            .filter(|r| {
                                with!(
                                    | level_r, category_r | * level_r == r.level && (r.category == *
                                    category_r || * category_r == Category::Any && r.category ==
                                    Category::P)
                                )
                            })
                            .cloned()
                            .collect::<Box<_>>(),
                    )}

                </tbody>
            </table>
        </div>
    }
}

// FIXME: inelegant, cloning, verbose, badly named, needs argument decopulation
fn levels_into_options(runs: &[Run], level: &str) -> Vec<HtmlElement<Option_>> {
    let mut a = runs.iter().map(|r| &r.level).cloned().collect::<Vec<_>>();
    a.sort();
    a.dedup();
    a.into_iter()
        .map(|l| {
            let lvl = l.clone();
            let level = String::from(level);
            // NOTE:
            // level.with(|level| *level ...)
            // differs from
            // level.with(|&level| level ...)
            // for `move` closures
            view! {
                <option value=l.clone() selected=move || level == lvl>
                    {l}
                </option>
            }
        })
        .collect()
}

// #[expect(
//     clippy::pattern_type_mismatch,
//     reason = "same-name variable deconstruction to references is not a type mismatch"
// )]
fn runs_into_trs(runs: &[Run]) -> Vec<HtmlElement<Tr>> {
    runs.iter()
        .enumerate()
        .map(|(idx, run)| {
            let Run {
                runner,
                igt_ms,
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
                // TODO: fix submission date display

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
            let _silencer = true.into_attribute();
            let proof = proof.to_string();
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
        .collect()
}
