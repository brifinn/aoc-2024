use day3::{get_input_from_file, mul_re, Symbol};
use std::process;

fn main() {
    let reports = get_input_from_file().unwrap_or_else(|err| {
        println!("Failed to get reports from file: {err:#?}");
        process::exit(1)
    });

    let mut sum: u32 = 0;
    let mut enable: bool = true;
    for s in mul_re().captures_iter(reports.as_str()).map(Symbol::from_cap) {
        match s {
            Symbol::DO => { enable = true; },
            Symbol::DONT => { enable = false; },
            Symbol::MUL(a, b) => {
                if enable {
                    sum += a * b
                }
            }
        }
    }
    dbg!(sum);
}
