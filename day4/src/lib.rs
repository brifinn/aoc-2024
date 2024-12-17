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

pub fn chexmas(matrix: &Vec<Vec<char>>, start: (isize, isize), direction: (isize, isize)) -> bool {
    let (mut row, mut col) = start;
    let (d_row, d_col) = direction;
    for c in ['X', 'M', 'A', 'S'] {
        if row < 0 || row >= matrix.len() as isize {
            return false;
        }
        if col < 0 || col >= matrix[row as usize].len()  as isize {
            return false;
        }
        // Lazy; assuming some bounds on size
        if matrix[row as usize][col as usize] != c {
            return false;
        }
        row += d_row;
        col += d_col;
    }
    true
}

pub fn make_cross_vectors(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> (Vec<char>, Vec<char>) {
    let mut backslash: Vec<char> = Vec::new();
    let mut forwardslash: Vec<char> = Vec::new();
    for i in 0..3 {
        backslash.push(matrix[row + i][col + i]);
        forwardslash.push(matrix[row + i][col + 2 - i]);
    }
    (backslash, forwardslash)
}

pub fn check_x_mas(backslash: &Vec<char>, forwardslash: &Vec<char>) -> bool {
    //eprintln!("backslash: {backslash:#?}, forward slash: {forwardslash:#?}");
    let mas = vec!['M', 'A', 'S'];
    let mut backslash_fore = backslash.iter();
    let mut backslash_aft = backslash.iter().rev();
    let mut forwardslash_fore = forwardslash.iter();
    let mut forwardslash_aft = forwardslash.iter().rev();
    let mut back_fore = true;
    let mut back_aft = true;
    let mut fore_fore = true;
    let mut fore_aft = true;
    for c in mas {
        back_fore &= *backslash_fore.next().unwrap() == c;
        back_aft &= *backslash_aft.next().unwrap() == c;
        fore_fore &= *forwardslash_fore.next().unwrap() == c;
        fore_aft &= *forwardslash_aft.next().unwrap() == c;
    }
    (back_fore || back_aft) && (fore_fore || fore_aft)
 }
