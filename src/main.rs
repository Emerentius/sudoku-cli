extern crate sudoku;
#[macro_use]
extern crate clap;

use sudoku::Sudoku;
use sudoku::parse_errors::LineFormatParseError;

#[cfg(feature = "rayon")]
extern crate rayon;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use std::io::{self, Read, Write};
use clap::{Arg, App, SubCommand};

enum Time {
    // only necessary for pre-computation, which is with rayon
    #[cfg_attr(not(feature = "rayon"), allow(dead_code))]
    Measured(std::time::Duration),
    DoMeasure(bool),
}

fn solve_and_print(input: &str, path: Option<&std::path::Path>) {
    let sudokus;
    #[cfg(not(feature = "rayon"))]
    {
        sudokus = input.lines()
            .map(sudoku::Sudoku::from_str_line)
            .map(|maybe_sudoku| {
                maybe_sudoku.map(|sudoku| sudoku.solve_unique().ok_or(sudoku))
            });
    }

#[cfg(feature = "rayon")]
    {
        sudokus = input.par_lines()
        .map(sudoku::Sudoku::from_str_line)
        .map(|maybe_sudoku| {
                maybe_sudoku.map(|sudoku| sudoku.solve_unique().ok_or(sudoku))
        })
            .collect::<Vec<_>>()
            .into_iter();
    }

    _print(sudokus, path);
}

fn solve_and_print_stats(input: &str, path: Option<&std::path::Path>, stats: bool) {
    let sudokus;
    let time2;
    #[cfg(not(feature = "rayon"))]
    {
        sudokus = input.lines()
        .map(sudoku::Sudoku::from_str_line)
        .map(|maybe_sudoku| {
                maybe_sudoku.map(|sudoku| sudoku.count_at_most(2))
        });

        time2 = Time::DoMeasure(stats);
    }

    #[cfg(feature = "rayon")]
    {
        use std::time::Instant;
        let start = match stats {
            true => Some(Instant::now()),
            false => None,
        };
        sudokus = input.par_lines()
            .map(sudoku::Sudoku::from_str_line)
            .map(|maybe_sudoku| {
                maybe_sudoku.map(|sudoku| sudoku.count_at_most(2))
            })
            .collect::<Vec<_>>()
            .into_iter();
        let duration = start.map(|start| Instant::now() - start);
        time2 = match duration {
            Some(duration) => Time::Measured(duration),
            None => Time::DoMeasure(false),
        }
    }

    _print_stats(sudokus, path, stats, time2);
}

fn _print<I: Iterator<Item=Result<Result<Sudoku, Sudoku>, LineFormatParseError>>>(sudokus: I, _path: Option<&std::path::Path>) {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    for sud in sudokus {
        match sud {
            Ok(Ok(solution)) => {
                let _ = writeln!(lock, "{}", solution.to_str_line());
            }
            Ok(Err(original)) => {
                let _ = writeln!(lock, "{} no unique solution", original.to_str_line());
            }
            Err(e) => {
                let _ = writeln!(lock, "invalid sudoku: {}", e);
            }
        };
    }
    /*if let Some(path) = path {
        println!("{}", path.display());
    }*/
}

fn _print_stats<I: Iterator<Item=Result<usize, LineFormatParseError>>>(sudokus: I, path: Option<&std::path::Path>, count: bool, time: Time) {
    use std::time::Instant;
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    let (mut n_solved, mut n_invalid, mut n_non_unique) = (0, 0, 0);

    let start = match time {
        Time::DoMeasure(true) => Some(Instant::now()),
        _ => None,
    };

    for sud in sudokus {
        match sud {
            Ok(0) => n_invalid += 1,
            Ok(1) => n_solved += 1,
            Ok(_) => n_non_unique += 1,
            Err(e) => {
                let _ = eprintln!("invalid sudoku: {}", e);
                n_invalid += 1;
            },
        };
    }

    let duration = start.map(|start| Instant::now() - start).or(match time {
        Time::Measured(duration) => Some(duration),
        _ => None,
    });

    let total = n_solved + n_invalid + n_non_unique;

    if count {
        let _ = write!(lock, "{:>9} {:>9} {:>9} {:>9} ", total, n_solved, n_non_unique, n_invalid);
    }
    if let Some(time_taken) = duration {
        let seconds = time_taken.as_secs() as f64 + time_taken.subsec_nanos() as f64 * 1e-9;
        let solving_rate = total as f64 / seconds;
        let _ = write!(lock, "{:>10.3} {:>10.0} ", seconds, solving_rate);
    }

    if let Some(path) = path {
        let _ = write!(lock, "{}", path.display());
    }
    let _ = write!(lock, "\n");
}

fn read_stdin(buffer: &mut String) {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    let _ = lock.read_to_string(buffer);
}

fn read_sudokus_and_execute<F>(
    matches: &clap::ArgMatches,
    mut callback: F,
)
where
    F: FnMut(Option<&std::path::Path>, &str)
{
    let mut sudoku_buffer = String::new();

    if let Some(filenames) = matches.values_of("sudokus_file") {
        for filename in filenames {
            let path = std::path::Path::new(filename);

            let mut file = match std::fs::File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    println!("Could not open file: {}", e);
                    return
                }
            };
            let _ = file.read_to_string(&mut sudoku_buffer);
            callback(Some(path), &sudoku_buffer);
            sudoku_buffer.clear();
        }
    } else {
        read_stdin(&mut sudoku_buffer);
        callback(None, &sudoku_buffer);
    }
}

fn main() {
    let mut app = App::new("sudoku")
        .version(crate_version!())
        .about("Solves and generates sudokus")
        .subcommand(
            SubCommand::with_name("solve")
                .about("Solve sudokus")
                .arg(
                    Arg::with_name("sudokus_file")
                        .takes_value(true)
                        .value_name("FILE")
                        .multiple(true)
                )
                .arg(
                    Arg::with_name("statistics")
                        .long("stat")
                        .short("s")
                        .help("do not print solutions, but categorize sudokus by solution count (1, >1, 0) and measure solving speed")
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
        )
        .subcommand(
            SubCommand::with_name("shuffle")
                .about("Performs symmetry transformations that result in a different but equivalent sudoku")
                .arg(
                    // TODO: Decide on how to unify amount (on generate option) and count
                    Arg::with_name("count")
                        .takes_value(true)
                        .short("n")
                )
                .arg(
                    Arg::with_name("sudokus_file")
                        .takes_value(true)
                        .value_name("FILE")
                        .multiple(true)
                )
        );
    let matches = app.clone().get_matches();

    // FIXME: don't read all sudokus into buffer
    //        read some in, process and output, read next in
    //let mut sudoku_buffer = String::new();

    if let Some(matches) = matches.subcommand_matches("solve") {
        let statistics = matches.is_present("statistics");

        // without printing solutions, print the header once
        // with solutions print it just before statistics
        if statistics {
            println!("{:>9} {:>9} {:>9} {:>9} {:>10} {:>10} ", "total", "unique", "nonunique", "invalid", "time [s]", "sudokus/s");
        }

        let action = |path: Option<&std::path::Path>, buffer: &str| {
            match statistics {
                false => solve_and_print(buffer, path),
                true => solve_and_print_stats(buffer, path, statistics),
            }
        };
        read_sudokus_and_execute(matches, action);
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
    } else if let Some(matches) = matches.subcommand_matches("shuffle") {
        let amount = value_t!(matches.value_of("count"), usize).unwrap_or(1);

        let action = |_: Option<&std::path::Path>, buffer: &str| {
            let stdout = std::io::stdout();
            let mut lock = stdout.lock();
            for sudoku in buffer.lines().map(Sudoku::from_str_line) {
                let mut sudoku = match sudoku {
                    Ok(s) => s,
                    Err(e) => {
                        let _ = eprintln!("invalid sudoku: {}", e);
                        continue
                    }
                };

                for _ in 0..amount {
                    sudoku.shuffle();
                    let _ = writeln!(lock, "{}", sudoku.to_str_line());
                }
            }
        };
        read_sudokus_and_execute(matches, action);
    } else {
        let _ = app.print_help();
    }
}
