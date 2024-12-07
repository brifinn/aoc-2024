use std::env;
use std::fs;

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

pub struct Report {
    levels: Vec<i32>
}

impl Report {
    pub fn build(report: &str) -> Report {
        let levels = report.split_whitespace()
            .map(|level| {level.parse::<i32>()})
            .into_iter()
            .flatten()  // I'm just giving up on error handling now; I had my fun on the first part
            .collect();
        Report { levels }
    }

    fn check_valid_increment(larger_value: i32, lower_value: i32) -> bool {
        // TODO larn me how overflow applies here
        return larger_value - lower_value >= 1 && larger_value - lower_value <= 3;
    }

    fn check_valid (&self, index_to_skip: Option<usize>) -> bool {
        let mut ascending: Option<bool> = None;
        let mut predecessor: Option<i32> = None;
        println!("Checking {:?}", self.levels);
        for pair in self.levels.iter().enumerate() {
            let (idx, num) = pair;
            match index_to_skip {
                Some(index) => {
                    if index == idx {
                        println!("Skipping idx {index}");
                        continue;
                    }
                }
                None => {},
            }

            // Not using get_or_insert because I want to short-circuit on None
            let last_num = match predecessor {
                None => {
                    predecessor = Some(*num);
                    continue;
                },
                Some(last_num) => last_num,
            };


            // Tricky: if last_num == num, this is invalid. But we'll catch it below
            // in the descending test anyway.
            let ascending = ascending.get_or_insert(*num > last_num);
            println!("Decided ascending should be {ascending:#?}");

            // Figure out when and why I have to dereference a scalar
            if *ascending && Report::check_valid_increment(*num, last_num){
                predecessor = Some(*num);
            } else if !*ascending && Report::check_valid_increment(last_num, *num) {
                predecessor = Some(*num)
            } else {
                println!("Invalid increment from {} to {}", last_num, *num);
                return false;
            }
        }
        true
    }

    pub fn check_valid_without_dampener(&self) -> bool {
        self.check_valid(None)
    }

    pub fn check_valid_with_dampener(&self) -> bool {
        for i in 0..self.levels.len() {
            if self.check_valid(Some(i)) {
                return true;
            }
        }
        false
    }
}

pub fn check_report(report_str: &str) -> bool {
    let report = Report::build(report_str);
    //report.check_valid_without_dampener()
    report.check_valid_with_dampener()
}

