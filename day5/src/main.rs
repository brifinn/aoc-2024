use day5::{get_input_from_file};
use std::process;

fn main() {
    let reports = get_input_from_file().unwrap_or_else(|err| {
        println!("Failed to get reports from file: {err:#?}");
        process::exit(1)
    });

    /*
    dict [num -> list of required predecessors, list of required successors ]

    for each element of list, get [list of predecessors, num, list of successors]
    look up required predecessors and successors
    establish that each successor is allowed
        consider a) required successors, b) required predecessors
    interesting: indirect requirements seem to require that the indirection be present, e.g.
        1|2
        2|3
    would seem to require that 1 precede 3, but no rule actually requires 1|3, so if 2 is absent, then 3, 1 is valid

    1
    for each predecessor, either in list of predecessors or (absent and not in list of successors)
    for each successor, either in list of successors or (absent and not in list of predecessors)

    or 2
    for each successor, that successor is not in list of required predecessors and this is not in list of successor's required successors
    ignore predecessors


     */
}
