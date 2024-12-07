use day2::{get_reports_from_file, check_report};
use std::process;

fn main() {
    let reports = get_reports_from_file().unwrap_or_else(|err| {
        println!("Failed to get reports from file: {err:#?}");
        process::exit(1)
    });

    let mut num_good_reports = 0;
    for line in reports.lines() {
        if check_report(line) {
            num_good_reports += 1;
        }
    }
    println!("{num_good_reports}");
}
