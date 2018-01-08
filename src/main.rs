extern crate sudoku;
#[macro_use]
extern crate clap;
extern crate isatty;

use sudoku::Sudoku;
use sudoku::parse_errors::LineFormatParseError;

#[cfg(feature = "rayon")]
extern crate rayon;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use std::io::{self, Read, Write};
use clap::{Arg, App, SubCommand};

#[cfg(feature = "rayon")]
fn solve_and_print(input: &str) {
    let vec = input.par_lines()
        .map(sudoku::Sudoku::from_str_line)
        .map(|maybe_sudoku| {
            maybe_sudoku.map(|sudoku| sudoku.solve_unique())
        })
        .collect::<Vec<_>>();


    _print(vec.into_iter());
}

#[cfg(not(feature = "rayon"))]
fn solve_and_print(input: &str) {
    let sudokus = input.lines()
        .map(sudoku::Sudoku::from_str_line)
        .map(|maybe_sudoku| {
            maybe_sudoku.map(|sudoku| sudoku.solve_unique())
        });

    _print(sudokus);
}

fn _print<I: Iterator<Item=Result<Option<Sudoku>, LineFormatParseError>>>(sudokus: I) {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for sud in sudokus {
        let _ = match sud {
            Ok(Some(solution)) => writeln!(lock, "{}", solution.to_str_line()),
            Ok(None) => write!(lock, "no unique solution\n"),
            Err(_) => write!(lock, "invalid sudoku\n"),
        };
    }
}

fn read_stdin(buffer: &mut String) {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    let _ = lock.read_to_string(buffer);
}

fn main() {
    let mut app = App::new("sudoku")
        .version("0.2")
        .about("Solves and generates sudokus")
        .subcommand(
            SubCommand::with_name("solve")
                .about("Solve sudokus")
                .arg(
                    Arg::with_name("sudokus_file")
                        .takes_value(true)
                        .value_name("FILE")
                )
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate sudokus")
                .arg(
                    Arg::with_name("amount")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("solved")
                        .help("generate solved sudokus")
                        .long("solved")
                )
        );
    let matches = app.clone().get_matches();

    // FIXME: don't read all sudokus into buffer
    //        read some in, process and output, read next in
    let mut sudoku_buffer = String::new();

    if let Some(matches) = matches.subcommand_matches("solve") {
        if let Some(filename) = matches.value_of("sudokus_file") {
            let mut file = match std::fs::File::open(filename) {
                Ok(f) => f,
                Err(e) => {
                    println!("Could not open file: {}", e);
                    return
                }
            };
            let _ = file.read_to_string(&mut sudoku_buffer);
        } else {
            read_stdin(&mut sudoku_buffer);
        }
        solve_and_print(&sudoku_buffer);
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        let amount = value_t_or_exit!(matches.value_of("amount"), usize);
        let gen_sud = match matches.is_present("solved") {
            true => Sudoku::generate_filled,
            false => Sudoku::generate_unique,
        };

        #[cfg(feature = "rayon")]
        (0..amount).into_par_iter()
            .for_each(|_| {
                println!("{}", gen_sud().to_str_line());
            });

        #[cfg(not(feature = "rayon"))]
        {
            let mut stdout = io::stdout();
            let mut lock = stdout.lock();
            for _ in 0..amount {
                let _ = writeln!(lock, "{}", gen_sud().to_str_line());
            }
        }
    } else if matches.subcommand.is_none() {
        if !isatty::stdin_isatty() {
            // if not operating interactively, read input from pipe and solve
            // this isn't advertised in the help but it'll be a common "error"
            // and be learned that way
            read_stdin(&mut sudoku_buffer);
            solve_and_print(&mut sudoku_buffer);
        } else {
            // print help
            let _ = app.print_help();
        }
    }
}
