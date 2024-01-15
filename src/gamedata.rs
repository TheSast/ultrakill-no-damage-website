use csv::ReaderBuilder;
use leptos::logging::error;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    P,
    Any,
    NoMo,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Passive,
    Lenient,
    Standard,
    Violent,
    Brutal,
    UltrakillMustDie,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Run {
    pub runner: String,
    // may include custom level names
    pub level: String,
    pub igt_ms: u32,
    pub category: Category,
    pub submission_date: u64,
    pub difficulty: Difficulty,
    pub patch_release_date: u64,
    pub proof: String,
}

#[allow(clippy::zero_prefixed_literal)]
pub fn load_runs() -> Vec<Run> {
    let mut file = match std::fs::File::open("assets/run_data.csv") {
        Ok(file) => file,
        Err(err) => {
            error!("Error opening file: {}", err);
            return Vec::new();
        }
    };
    let mut csv_data = String::new();
    if let Err(err) = std::io::Read::read_to_string(&mut file, &mut csv_data) {
        error!("Error reading file: {}", err);
        return Vec::new();
    }
    let runs = parse_csv(&csv_data).unwrap_or_else(|err| {
        error!("Error parsing CSV: {err}\n ```csv{csv_data:?}```");
        return Vec::new();
    });
    runs
    // vec![
    //     Run {
    //         runner: String::from("TheSast"),
    //         igt_ms: 045_377,
    //         submission_date: 1666526400,
    //         difficulty: Difficulty::Violent,
    //         patch_release_date: 1660651200,
    //     },
    //     Run {
    //         runner: String::from("Westaxle"),
    //         igt_ms: 002_377,
    //         submission_date: 1666526900,
    //         difficulty: Difficulty::Standard,
    //         patch_release_date: 1660651200,
    //     },
    // ]
}

fn parse_csv(csv_data: &str) -> Result<Vec<Run>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());

    let mut runs = vec![];

    for result in reader.records() {
        let record = result?;

        let runner = record.get(0).ok_or("Missing runner")?.trim().to_string();
        let level = record.get(1).ok_or("Missing level")?.trim().to_string();
        let igt_ms = record
            .get(2)
            .ok_or("Missing igt_ms")?
            .trim()
            .replace("_", "")
            .parse::<u32>()
            .map_err(|_| "Invalid digit found in igt_ms")?;
        let category = match record.get(3).ok_or("Missing category")?.trim() {
            "P" => Category::P,
            "Any" => Category::Any,
            "NoMo" => Category::NoMo,
            _ => return Err("Invalid category".into()),
        };
        let submission_date = record
            .get(4)
            .ok_or("Missing submission_date")?
            .trim()
            .parse::<u64>()
            .map_err(|_| "Invalid digit found in submission_date")?;
        let difficulty = match record.get(5).ok_or("Missing difficulty")?.trim() {
            "Passive" => Difficulty::Passive,
            "Lenient" => Difficulty::Lenient,
            "Standard" => Difficulty::Standard,
            "Violent" => Difficulty::Violent,
            "Brutal" => Difficulty::Brutal,
            "UltrakillMustDie" => Difficulty::UltrakillMustDie,
            _ => return Err("Invalid difficulty".into()),
        };
        let patch_release_date = record
            .get(6)
            .ok_or("Missing patch_release_date")?
            .trim()
            .parse::<u64>()
            .map_err(|_| "Invalid digit found in patch_release_date")?;
        let proof = record.get(7).ok_or("Missing proof")?.trim().to_string();

        let run = Run {
            runner,
            level,
            igt_ms,
            category,
            submission_date,
            difficulty,
            patch_release_date,
            proof,
        };

        runs.push(run);
    }

    Ok(runs)
}