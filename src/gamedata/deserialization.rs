use super::{Act, Category, Datetime, Deserialize, Difficulty, Layer, Level, Patch, Run, Track};
use std::error::Error;

#[derive(Deserialize)]
struct IndividualLevelRun {
    runner: String,
    track: Level,
    igt_ms: u32,
    category: Category,
    submission_date: Datetime,
    difficulty: Difficulty,
    patch_release_date: Patch,
    proof: String,
}

impl From<IndividualLevelRun> for Run {
    fn from(item: IndividualLevelRun) -> Self {
        Self {
            runner: item.runner,
            track: Track::Level(item.track),
            igt_ms: item.igt_ms,
            category: item.category,
            submission_date: item.submission_date,
            difficulty: item.difficulty,
            patch_release_date: item.patch_release_date,
            proof: item.proof,
        }
    }
}

impl From<IndividualLevelRun> for Vec<Run> {
    fn from(item: IndividualLevelRun) -> Self {
        vec![item.into()]
    }
}

#[derive(Deserialize)]
struct LayerRun {
    runner: String,
    track: Layer,
    igt_ms: u32,
    category: Category,
    submission_date: Datetime,
    difficulty: Difficulty,
    patch_release_date: Patch,
    proof: String,
    levels: Vec<IndividualLevelRun>,
}

impl From<LayerRun> for Vec<Run> {
    fn from(item: LayerRun) -> Self {
        item.levels
            .into_iter()
            .flat_map(Into::<Self>::into)
            .chain([Run {
                runner: item.runner,
                track: Track::Layer(item.track),
                igt_ms: item.igt_ms,
                category: item.category,
                submission_date: item.submission_date,
                difficulty: item.difficulty,
                patch_release_date: item.patch_release_date,
                proof: item.proof,
            }])
            .collect()
    }
}

#[derive(Deserialize)]
struct ActRun {
    runner: String,
    track: Act,
    igt_ms: u32,
    category: Category,
    submission_date: Datetime,
    difficulty: Difficulty,
    patch_release_date: Patch,
    proof: String,
    layers: Vec<LayerRun>,
}

impl From<ActRun> for Vec<Run> {
    fn from(item: ActRun) -> Self {
        item.layers
            .into_iter()
            .flat_map(Into::<Self>::into)
            .chain([Run {
                runner: item.runner,
                track: Track::Act(item.track),
                igt_ms: item.igt_ms,
                category: item.category,
                submission_date: item.submission_date,
                difficulty: item.difficulty,
                patch_release_date: item.patch_release_date,
                proof: item.proof,
            }])
            .collect()
    }
}

#[derive(Deserialize)]
struct FullgameRun {
    runner: String,
    igt_ms: u32,
    category: Category,
    submission_date: Datetime,
    difficulty: Difficulty,
    patch_release_date: Patch,
    proof: String,
    acts: Vec<ActRun>,
}

impl From<FullgameRun> for Vec<Run> {
    fn from(item: FullgameRun) -> Self {
        item.acts
            .into_iter()
            .flat_map(Into::<Self>::into)
            .chain([Run {
                runner: item.runner,
                track: Track::Fullgame,
                igt_ms: item.igt_ms,
                category: item.category,
                submission_date: item.submission_date,
                difficulty: item.difficulty,
                patch_release_date: item.patch_release_date,
                proof: item.proof,
            }])
            .collect()
    }
}

// it will kind of `match` against possible variants, so the easiest to match should go last
#[derive(Deserialize)]
#[serde(untagged)]
enum RunFakeUnion {
    LayerRun(LayerRun),
    ActRun(ActRun),
    Fullgame(FullgameRun),
    IndividualLevelRun(IndividualLevelRun),
}

impl From<RunFakeUnion> for Vec<Run> {
    fn from(item: RunFakeUnion) -> Self {
        match item {
            RunFakeUnion::LayerRun(v) => v.into(),
            RunFakeUnion::ActRun(v) => v.into(),
            RunFakeUnion::Fullgame(v) => v.into(),
            RunFakeUnion::IndividualLevelRun(v) => v.into(),
        }
    }
}

// TODO: move away from Box<dyn Error> once the errortype of toml::from_str is undrestood
pub fn parse_toml(toml_data: &str) -> Result<Vec<Run>, Box<dyn Error>> {
    use std::collections::HashMap;
    // TODO:
    // 1. Validate that there are no duplicate runs
    // 2. Validate that proof.starts_with("https://")
    // 3. Validate that layer/act/fullgame runs contain one of each the required level/later/act runs
    // 4. Validate that layer/act/fullgame runs time is the sum of it's level/later/act runs
    // 5. Validate that layer/act/fullgame runs category is less or equally restrictive to it's level/later/act runs
    // 6. Validate that submission_date.date.is_some()
    // 7. Use MaybePatch
    // 8? Validate that !(runner.contains("http://") || runner.contains("https://"))
    // 9. Validate that there are no tracks that could not be ran during the specified patch or
    //    submission date
    // 10. Validate that super::Level::Custom() is defined in toml.remove("custom-levels")
    Ok(
        toml::from_str::<HashMap<String, Vec<RunFakeUnion>>>(toml_data)?
            .remove("runs")
            .ok_or("Missing \"runs\"")?
            .into_iter()
            .flat_map(Into::<Vec<Run>>::into)
            .collect(),
    )
}
