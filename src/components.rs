#![expect(
    clippy::empty_structs_with_brackets,
    clippy::infinite_loop,
    clippy::module_name_repetitions,
    reason = "Leptos components do not appropriately allow or expect lints"
)]
mod leaderboard;
pub use leaderboard::Leaderboard;
