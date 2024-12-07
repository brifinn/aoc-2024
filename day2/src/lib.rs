use std::env;
use std::fs;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

pub fn get_reports_from_file() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(format!("Not enough arguments; got {args:#?}"));
    }

    let fname = &args[1];
/*     let reports = fs::read_to_string(fname).unwrap_or_else(|err| {
        return Err(format!("Couldn't read report file {fname}:\n{err:#?}"));
    }); */
    let reports = match fs::read_to_string(fname) {
        Ok(reports) => reports,
        Err(e) => return Err(format!("Couldn't read report file {fname}:\n{e:#?}")),
    };

    Ok(reports)
}

pub enum ReportState {
    Uninitialized,
    Undirected(i32),
    Ascending(i32),
    Descending(i32),
}

impl ReportState {
    fn check_valid_increment(larger_value: i32, lower_value: i32, min_increment: i32, max_increment: i32) -> bool {
        // TODO larn me how overflow applies here
        return larger_value - lower_value >= min_increment && larger_value - lower_value <= max_increment;
    }

    fn transition(&self, new_level: i32) -> Result<ReportState, &'static str>{
        match self {
            ReportState::Uninitialized => Ok(ReportState::Undirected(new_level)),
            ReportState::Undirected(last_level) => {
                if new_level > *last_level && ReportState::check_valid_increment(new_level, *last_level, 1, 3) {
                    Ok(ReportState::Ascending(new_level))
                } else if new_level < *last_level && ReportState::check_valid_increment(*last_level, new_level, 1, 3) {
                    Ok(ReportState::Descending(new_level))
                } else {
                    Err("Second level same as the first")
                }
            },
            ReportState::Ascending(last_level) => {
                if ReportState::check_valid_increment(new_level, *last_level, 1, 3) {
                    Ok(ReportState::Ascending(new_level))
                } else {
                    Err("Ascending series but increment out of bounds")
                }
            },
            ReportState::Descending(last_level) => {
                if ReportState::check_valid_increment(*last_level, new_level, 1, 3) {
                    Ok(ReportState::Descending(new_level))
                } else {
                    Err("Descending series but increment out of bounds")
                }
            },
        }
    }
}

pub fn check_report(report: &str) -> Result<ReportState, &'static str> {
    report.split_whitespace()
          .map(|level| {level.parse::<i32>()})
          .fold_while(Ok(ReportState::Uninitialized), |state, level| {
              // Is there an unwrappy idiom for this, or is fold_while messing me up here?
              // The problem is that I want to return a Result from the closure, but fold_while templatizes
              // the return type from the input type, so I have to take a Result as an input, even though I'll
              // use Continue/Done to prevent ever running this on an Err.
              let state: ReportState = match state {
                  Ok(state) => state,
                  Err(_) => { return Done(Err("Shouldn't get here"))}
              };

              let this_level: i32 = match level {
                  Ok(i) => i,
                  Err(e) => {
                      eprintln!("Failed to parse level with {e:#?}");
                      // Tricky: using ReportState::transition, which returns a Result<ReportState, &str>,
                      // means I can't use the error from int parsing directly, because it is a ParseIntError,
                      // and fold_while templatized the closure with the return type inferred from transition.
                      // I tried putting a dynamic error type on transition, but that made me jump through hoops
                      // on my string literal errors themselves. Easier to return a string error here, since I
                      // know well enough why it happened. Ugh, and this makes me return a str error too. Need
                      // to figure out trait objects
                      return Done(Err("Ain't parse int from string"));
                  },
              };

              match state.transition(this_level) {
                  Ok(new_state) => {
                    println!("Report {report} is ok");
                    Continue(Ok(new_state))
                  },
                  Err(e) => {
                    eprintln!("Report {report} is bad");
                    Done(Err(e))
                  },
              }
          }).into_inner()
}

