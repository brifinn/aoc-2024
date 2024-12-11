use day2::{get_input_from_file, matricize, chexmas};
use std::process;

fn main() {
    let reports = get_input_from_file().unwrap_or_else(|err| {
        println!("Failed to get reports from file: {err:#?}");
        process::exit(1)
    });

    let matrix = matricize(reports);
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

    let mut instances: usize = 0;
    for row in (0..matrix.len()) {
        for col in (0..matrix[row].len()) {
            for direction in directions {
                if chexmas(&matrix, (row, col), direction) {
                    instances += 1;
                }
            }
        }
    }
    println!("{instances} matches");
}
