use std::env;
use std::fs;

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

pub fn matricize(input: &String) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        matrix.push(line.chars().collect());
    }
    matrix
}

pub fn chexmas(matrix: &Vec<Vec<char>>, start: (usize, usize), direction: (isize, isize)) -> bool {
    let (mut row, mut col) = start;
    let (d_row, d_col) = direction;
    for c in ["X", "M", "A", "S"] {
        if row < 0 || row > matrix.len() {
            return false;
        }
        if col < 0 || col > matrix[row].len() {
            return false;
        }
        if matrix[row][col] != c {
            return false;
        }
        row += d_row;
        col += d_col;
    }
    true
}
