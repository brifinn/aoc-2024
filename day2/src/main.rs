use day2::{get_reports_from_file, check_report};
use std::process;

fn main() {
    let reports = get_reports_from_file().unwrap_or_else(|err| {
        println!("Failed to get reports from file: {err:#?}");
        process::exit(1)
    });

    let num_good_reports = reports.lines().map(check_report).into_iter().flatten().count();
    println!("{num_good_reports}");
}
