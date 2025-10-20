use clap::{Arg, ArgAction, Command};
use regex::RegexBuilder;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

struct Options {
    ignore_case: bool,
    show_lineno: bool,
    invert: bool,
    recursive: bool,
    count: bool,
}

fn build_cli() -> Command {
    Command::new("simplegrep")
        .arg(Arg::new("pattern").required(true).index(1))
        .arg(Arg::new("paths").num_args(0..).index(2))
        .arg(Arg::new("ignore-case").short('i').long("ignore-case").action(ArgAction::SetTrue))
        .arg(Arg::new("line-number").short('n').long("line-number").action(ArgAction::SetTrue))
        .arg(Arg::new("invert-match").short('v').long("invert-match").action(ArgAction::SetTrue))
        .arg(Arg::new("recursive").short('r').long("recursive").action(ArgAction::SetTrue))
        .arg(Arg::new("count").short('c').long("count").action(ArgAction::SetTrue))
}

fn process_file(path: &Path, re: &regex::Regex, opts: &Options) -> io::Result<i64> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut matches = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let matched = re.is_match(&line);
        let condition = if opts.invert { !matched } else { matched };

        if condition {
            matches += 1;
            if !opts.count {
                if opts.show_lineno {
                    println!("{}:{}:{}", path.display(), i + 1, line);
                } else {
                    println!("{}:{}", path.display(), line);
                }
            }
        }
    }
    Ok(matches)
}

fn process_path(path: &Path, re: &regex::Regex, opts: &Options) -> io::Result<i64> {
    if path.is_dir() && opts.recursive {
        let mut total = 0;
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                total += process_file(entry.path(), re, opts)?;
            }
        }
        Ok(total)
    } else if path.is_file() {
        process_file(path, re, opts)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Invalid path"))
    }
}

fn main() -> io::Result<()> {
    let matches = build_cli().get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let paths: Vec<String> = matches
        .get_many::<String>("paths")
        .map(|vals| vals.map(|s| s.to_owned()).collect())
        .unwrap_or_else(Vec::new);

    let opts = Options {
        ignore_case: matches.get_flag("ignore-case"),
        show_lineno: matches.get_flag("line-number"),
        invert: matches.get_flag("invert-match"),
        recursive: matches.get_flag("recursive"),
        count: matches.get_flag("count"),
    };

    let mut builder = RegexBuilder::new(pattern);
    builder.case_insensitive(opts.ignore_case);
    let re = builder.build().unwrap();

    if paths.is_empty() {
        let stdin = io::stdin();
        let reader = stdin.lock();
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if re.is_match(&line) {
                if opts.show_lineno {
                    println!("{}:{}", i + 1, line);
                } else {
                    println!("{}", line);
                }
            }
        }
    } else {
        for p in paths {
            let path = Path::new(&p);
            match process_path(path, &re, &opts) {
                Ok(_) => (),
                Err(e) => eprintln!("err {}: {}", p, e),
            }
        }
    }

    Ok(())
}

