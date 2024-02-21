use csv::ReaderBuilder;
use leptos::logging::error;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    P,
    Any,
    NoMo,
}

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Harmless,
    Lenient,
    Standard,
    Violent,
    Brutal,
    UltrakillMustDie,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
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

pub fn load_runs() -> Vec<Run> {
    // let mut file = match std::fs::File::open("assets/run_data.csv") {
    //     Ok(file) => file,
    //     Err(err) => {
    //         error!("Error opening file: {err}");
    //         return Vec::new();
    //     }
    // };
    // let mut csv_data = String::new();
    // if let Err(err) = std::io::Read::read_to_string(&mut file, &mut csv_data) {
    //     error!("Error reading file: {err}");
    //     return Vec::new();
    // }
    // I have no idea how to read files at runtime
    // HACK: embed the file in the binary
    let csv_data = std::include_str!("../assets/run_data.csv");
    parse_csv(csv_data).unwrap_or_else(|err| {
        error!("Error parsing CSV: {err}\nCSV: {csv_data:?}");
        Vec::new()
    })
}

fn parse_csv(csv_data: &str) -> Result<Vec<Run>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());

    let mut runs = vec![];

    for result in reader.records() {
        let record = result?;

        let runner = record.get(0).ok_or("Missing runner")?.trim().to_owned();
        let level = record.get(1).ok_or("Missing level")?.trim().to_owned();
        let igt_ms = record
            .get(2)
            .ok_or("Missing igt_ms")?
            .trim()
            .replace('_', "")
            .parse::<u32>()
            .map_err(|_e| "Invalid digit found in igt_ms")?;
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
            .map_err(|_e| "Invalid digit found in submission_date")?;
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
            .map_err(|_e| "Invalid digit found in patch_release_date")?;
        let proof = record.get(7).ok_or("Missing proof")?.trim().to_owned();

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
