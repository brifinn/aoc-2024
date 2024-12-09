use day3::{get_input_from_file, mul_re};
use std::process;

fn main() {
    let reports = get_input_from_file().unwrap_or_else(|err| {
        println!("Failed to get reports from file: {err:#?}");
        process::exit(1)
    });

    for mul in mul_re().captures_iter(reports) {
        println!("{mul:#?}");
    }
}
