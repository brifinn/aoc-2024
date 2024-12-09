use std::env;
use std::fs;
use regex::{Regex, RegexBuilder};

const MUL_RE: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

pub fn get_input_from_file() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(format!("Not enough arguments; got {args:#?}"));
    }

    let fname = &args[1];
    let reports = match fs::read_to_string(fname) {
        Ok(reports) => reports,
        Err(e) => return Err(format!("Couldn't read report file {fname}:\n{e:#?}")),
    };

    Ok(reports)
}

pub fn mul_re() -> Regex {
    RegexBuilder::new(MUL_RE)
        .multi_line(true)
        .build()
        .unwrap() // error handling?
}
/*
How do I define an iterator in rust? Guess I don't need to; the 
search input string w/ multiline enabled for mul\((\d?1:3),(\d?1:3)\) and parse out pairs
multiply pairs and add to accumulator
 */