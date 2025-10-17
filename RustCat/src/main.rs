use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process;

struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool,
    show_ends: bool,
    squeeze_blank: bool,
}

impl Config {
    fn from_args(args: Vec<String>) -> Result<Config, &'static str> {
        let mut files = Vec::new();
        let mut number_lines = false;
        let mut number_nonblank = false;
        let mut show_ends = false;
        let mut squeeze_blank = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-n" | "--number" => number_lines = true,
                "-b" | "--number-nonblank" => number_nonblank = true,
                "-E" | "--show-ends" => show_ends = true,
                "-s" | "--squeeze-blank" => squeeze_blank = true,
                "-h" | "--help" => {
                    print_help();
                    process::exit(0);
                }
                arg if arg.starts_with('-') => {
                    return Err("Unknown option");
                }
                _ => files.push(args[i].clone()),
            }
            i += 1;
        }

        if files.is_empty() {
            files.push("-".to_string());
        }

        Ok(Config {
            files,
            number_lines,
            number_nonblank,
            show_ends,
            squeeze_blank,
        })
    }
}

fn print_help() {
    println!("Usage: cat [OPTION]... [FILE]...");
    println!("Concatenate FILE(s) to standard output.");
    println!();
    println!("With no FILE, or when FILE is -, read standard input.");
    println!();
    println!("Options:");
    println!("  -n, --number             number all output lines");
    println!("  -b, --number-nonblank    number nonempty output lines");
    println!("  -E, --show-ends          display $ at end of each line");
    println!("  -s, --squeeze-blank      suppress repeated empty output lines");
    println!("  -h, --help               display this help and exit");
}

fn cat(config: Config) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut line_number = 1;
    let mut prev_blank = false;

    for filename in &config.files {
        let reader: Box<dyn BufRead> = if filename == "-" {
            Box::new(BufReader::new(io::stdin()))
        } else {
            Box::new(BufReader::new(File::open(filename)?))
        };

        for line in reader.lines() {
            let line = line?;
            let is_blank = line.trim().is_empty();

            if config.squeeze_blank && is_blank && prev_blank {
                continue;
            }
            prev_blank = is_blank;
            if config.number_nonblank && !is_blank {
                write!(handle, "{:6}\t", line_number)?;
                line_number += 1;
            } else if config.number_lines {
                write!(handle, "{:6}\t", line_number)?;
                line_number += 1;
            }

            write!(handle, "{}", line)?;

            if config.show_ends {
                write!(handle, "$")?;
            }

            writeln!(handle)?;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_args(args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        eprintln!("Try 'cat --help' for more information.");
        process::exit(1);
    });

    if let Err(e) = cat(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
