extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate structopt;

use csv::Reader;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Toggle Report",
    about = "Parses a csv and consolodates some fields into a report."
)]
struct Cli {
    /// The csv to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug, Deserialize)]
struct Record {
    user: String,
    email: String,
    client: String,
    project: String,
    task: String,
    description: String,
    billable: String,
    start_date: String,
    end_date: String,
    start_time: String,
    end_time: String,
    duration: String,
    tags: String,
    amount: String,
}

fn main() {
    let args = Cli::from_args();

    let mut work = HashMap::new();

    let mut rdr = Reader::from_path(&args.path).unwrap();
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        let project = record.project;

        let duration = String::from(record.duration);
        let v: Vec<&str> = duration.split(':').collect();
        let hours_as_minutes = v[0].parse::<i32>().unwrap() * 60;
        let minutes = v[1].parse::<i32>().unwrap() + hours_as_minutes;

        let project_entry = work.entry(project).or_insert(HashMap::<String, i32>::new());
        *project_entry.entry(record.description).or_insert(0) += minutes;
    }

    let col_width = 50;

    for (key, val) in work.iter() {
        let mut total_time = 0;
        for t in val.values() {
            total_time += t;
        }
        println!(
            "{}: {:.>3$}:{}",
            key,
            total_time / 60,
            total_time % 60,
            col_width - key.len()
        );
        for (subkey, subval) in val.iter() {
            println!(
                "     {}: {:.>3$}:{}",
                subkey,
                subval / 60,
                subval % 60,
                col_width - subkey.len() - 5
            );
        }
    }
}
