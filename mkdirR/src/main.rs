use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Використання: mkdir <шлях> [--parents]");
        std::process::exit(1);
    }

    let mut paths = Vec::new();
    let mut create_parents = false;

    // парсинг аргументів
    for arg in &args[1..] {
        if arg == "--parents" || arg == "-p" {
            create_parents = true;
        } else {
            paths.push(arg);
        }
    }

    for path in paths {
        if create_parents {
            fs::create_dir_all(path)?;
            println!("Створено (усі підкаталоги): {}", path);
        } else {
            fs::create_dir(path)?;
            println!("Створено: {}", path);
        }
    }

    Ok(())
}

