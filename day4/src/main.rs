use day4::{check_x_mas, chexmas, get_input_from_file, make_cross_vectors, matricize};
use std::process;

fn main() {
    let reports = get_input_from_file().unwrap_or_else(|err| {
        println!("Failed to get reports from file: {err:#?}");
        process::exit(1)
    });

    let matrix = matricize(&reports);
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut xmas_instances: usize = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            for direction in directions {
                if chexmas(&matrix, (row as isize, col as isize), direction) {
                    xmas_instances += 1;
                }
            }
        }
    }
    println!("{xmas_instances} xmas matches");

    let mut x_mas_instances: usize = 0;
    for row in 2..matrix.len()-2 {
        for col in 2..matrix[row].len()-2 {
            let (backslash, forwardslash) = make_cross_vectors(&matrix, row, col);
            if check_x_mas(&backslash, &forwardslash) {
                x_mas_instances += 1
            }
        }
    }
    println!("{x_mas_instances} x-mas matches");
}
