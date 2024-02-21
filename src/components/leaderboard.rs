use crate::gamedata::{self, Category, Run, Track};
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
// 2. Add alternative sorting options:
// - date
// - release date
#[component]
pub fn Leaderboard() -> impl IntoView {
    let runs = {
        let mut runs = match gamedata::load_runs() {
            Ok(v) => v,
            Err(e) => {
                return view! {
                    <div class="leaderboard">
                        <p class="error">"Failed to leaderboard with error: " {e.to_string()}</p>
                    </div>
                };
            }
        };
        runs.sort_by(|a, b| {
            a.track
                .cmp(&b.track)
                .then(a.igt_ms.cmp(&b.igt_ms))
                .then(b.difficulty.cmp(&a.difficulty))
                .then(b.patch_release_date.cmp(&a.patch_release_date))
                .then(a.submission_date.cmp(&b.submission_date))
                .then(a.runner.cmp(&b.runner))
                .then_with(|| panic!("Duplicate runs {a:?} {b:?}"))
        });
        runs
    };
    let (category_r, category_w) = create_signal(Category::Any);
    let (track_r, track_w) = create_signal(runs[0].track.to_string());
    // INFO: need to call `into_class` manually to silence `unused_import` warning
    let _silencer = true.into_class();
    view! {
        <div class="leaderboard">
            <div class="controls">
                <div>
                    <div>
                        <select on:change=move |ev| {
                            track_w.set(event_target_value(&ev));
                        }>

                            {
                                let runs = runs.clone();
                                move || tracks_into_options(
                                    runs.iter().map(|r| r.track.clone()).collect::<Vec<_>>(),
                                    &track_r.get(),
                                )
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
                                    | track_r, category_r | * track_r == r.track.to_string() && (r
                                    .category == * category_r || * category_r == Category::Any && r
                                    .category == Category::P)
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

fn tracks_into_options(mut tracks: Vec<Track>, track: &str) -> Vec<HtmlElement<Option_>> {
    use std::string;
    tracks.sort_by(|a, b| b.shallow_cmp(a)); // method chaining be in shambles rn
    tracks.dedup();
    tracks
        .iter()
        .map(string::ToString::to_string)
        .map(|t| {
            let trk = t.clone();
            let track = track.to_owned();
            // NOTE:
            // track.with(|track| *track ...)
            // differs from
            // track.with(|&track| track ...)
            // for `move` closures
            view! {
                <option value=t.clone() selected=move || track == trk>
                    {t}
                </option>
            }
        })
        .collect()
}

// #[allow(clippy::pattern_type_mismatch)] // reason = "same-name variable deconstruction to references is not a type mismatch"
fn runs_into_trs(runs: &[Run]) -> Vec<HtmlElement<Tr>> {
    runs.iter()
        .enumerate()
        .map(
            |(
                idx,
                Run {
                    runner,
                    igt_ms,
                    submission_date,
                    difficulty,
                    patch_release_date,
                    proof,
                    ..
                },
            )| {
                // let  = run;
                let igt_ms = format!(
                    "{:02}:{:02}:{:03}",
                    igt_ms / 1000 / 60,
                    igt_ms / 1000 % 60,
                    igt_ms % 1000
                );
                let submission_date = {
                    use toml::value::Date; // implements Display
                    use web_time::{Duration, SystemTime};
                    let seconds = {
                        let date = submission_date.date.unwrap_or(Date {
                            year: 0,
                            day: 0,
                            month: 0,
                        });
                        match (u64::from(date.year - 1970 /* fallible */) * 365 /* inaccurancy */ * 24 * 60 * 60)
                            + (u64::from(date.month) * 30 /* inaccurancy */ * 24 * 60 * 60)
                            + (u64::from(date.day) * 24 * 60 * 60)
                        {
                            0 => 0,
                            d => SystemTime::now()
                                .duration_since(
                                    SystemTime::UNIX_EPOCH + Duration::from_secs(d),
                                )
                                .as_ref()
                                .map(Duration::as_secs)
                                .unwrap_or(0),
                        }
                    };
                    let minutes = seconds / 60;
                    let hours = minutes / 60;
                    let days = hours / 24;
                    let months = days / 30;
                    let years = days / 365;

                    let plur = |i| if i > 1 { "s" } else { "" };
                    match (years, months, days, hours, minutes, seconds) {
                        (y, _, _, _, _, _) if y > 0 => {
                            format!("{} year{} ago", y, plur(y))
                        }
                        (_, m, _, _, _, _) if m > 0 => {
                            format!("{} month{} ago", m, plur(m))
                        }
                        (_, _, d, _, _, _) if d > 0 => {
                            format!("{} day{} ago", d, plur(d))
                        }
                        (_, _, _, h, _, _) if h > 0 => {
                            format!("{} hour{} ago", h, plur(h))
                        }
                        (_, _, _, _, m, _) if m > 0 => {
                            format!("{} minute{} ago", m, plur(m))
                        }
                        (_, _, _, _, _, s) if s > 0 => {
                            format!("{} second{} ago", s, plur(s))
                        }
                        _ => String::from("unknown"),
                    }
                };
                let difficulty = difficulty.to_string();
                // TODO: implement patch type
                // let patch_release_date = patch_release_date.to_string();
                // INFO: need to call `into_attribute` manually to silence `unused_import` warning
                let _silencer = true.into_attribute();
                let proof = proof.to_string();
                // TODO: use gamedata::loead_runners().get(runner).unwrap_or_else(|e| ...)
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
            },
        )
        .collect()
}
