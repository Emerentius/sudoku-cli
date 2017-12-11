extern crate sudoku;
extern crate clap;

extern crate rayon;
use rayon::prelude::*;

use std::io::{self, Read, Write};
use clap::{Arg, App};

fn solve_and_print(input: &str) {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    let vec = input.par_lines()
        .map(sudoku::Sudoku::from_str_line)
        .map(|maybe_sudoku| {
            maybe_sudoku.map(|sudoku| sudoku.solve_one())
        })
        .collect::<Vec<_>>();


    for sud in vec {
        let _ = match sud {
            Ok(Some(solution)) => write!(lock, "{}\n", solution.to_str_line()),
            Ok(None) => write!(lock, "no solution\n"),
            Err(_) => write!(lock, "invalid sudoku\n"),
        };
    }
}

fn main() {
    let matches = App::new("sudoku")
        .version("0.1")
        .about("Solves sudokus")
        .arg(Arg::with_name("sudoku_file")
            .long("file")
            .short("f")
            .takes_value(true)
            .value_name("FILE")
        )
        .get_matches();

    let mut sudoku_buffer = String::new();

    if let Some(filename) = matches.value_of("sudoku_file") {
        let mut file = std::fs::File::open(filename).unwrap_or_else(|e| panic!("Could not open file: {}", e));
        let _ = file.read_to_string(&mut sudoku_buffer);
    } else {
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        let _ = lock.read_to_string(&mut sudoku_buffer);
    }
    solve_and_print(&sudoku_buffer);
}
