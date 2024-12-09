use std::env;
use std::fs;
use regex::{Regex, RegexBuilder, Captures, Match};

const MUL_RE: &str = r"(?:mul\((\d{1,3}),(\d{1,3})\))|(?:(do)\(\))|(?:(don't)\(\))";

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

pub enum Symbol {
    DO,
    DONT,
    MUL(u32, u32)
}

impl Symbol {
    pub fn from_cap(cap: Captures) -> Symbol {
        //let first_match = cap.get(1).expect("All of these have a first capture").as_str();
        let captures: Vec<Match> = cap.iter().flatten().collect();
        let first_match = captures[1].as_str();

        match first_match {
            "do" => Symbol::DO,
            "don't" => Symbol::DONT,
            a => {
                let b = captures[2].as_str();
                let a: u32 = a.parse().expect("How are you not a u32");
                let b: u32 = b.parse().expect("bro come on");
                Symbol::MUL(a, b)
            }
        }
    }
}
