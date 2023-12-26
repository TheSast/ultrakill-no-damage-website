use csv::ReaderBuilder;
use leptos::logging::error;
use std::error::Error;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Run {
    pub runner: String,
    pub igt_ms: u32,
    pub submission_date: u64,
    pub difficulty: Difficulty,
    pub patch_release_date: u64,
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

        let runner = record.get(0).ok_or("Missing runner")?.to_string();
        let igt_ms = record.get(1).ok_or("Missing igt_ms")?.replace("_", "");
        let igt_ms = igt_ms.parse::<u32>()?;
        let submission_date = record
            .get(2)
            .ok_or("Missing submission_date")?
            .parse::<u64>()?;
        let difficulty = match record.get(3).ok_or("Missing difficulty")? {
            "Passive" => Difficulty::Passive,
            "Lenient" => Difficulty::Lenient,
            "Standard" => Difficulty::Standard,
            "Violent" => Difficulty::Violent,
            "Brutal" => Difficulty::Brutal,
            "UltrakillMustDie" => Difficulty::UltrakillMustDie,
            _ => return Err("Invalid difficulty".into()),
        };
        let patch_release_date = record
            .get(4)
            .ok_or("Missing patch_release_date")?
            .parse::<u64>()?;

        let run = Run {
            runner,
            igt_ms,
            submission_date,
            difficulty,
            patch_release_date,
        };

        runs.push(run);
    }

    Ok(runs)
}
