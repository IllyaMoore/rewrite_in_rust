use std::env;
use std::fs;
use std::path::Path;

fn find_in_dir(path: &Path, pattern: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                find_in_dir(&entry_path, pattern);
            } else if let Some(name) = entry_path.file_name() {
                if name.to_string_lossy().contains(pattern) {
                    println!("{}", entry_path.display());
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: pfind <directory> <pattern>");
        return;
    }

    let dir = Path::new(&args[1]);
    let pattern = &args[2];

    if !dir.exists() {
        eprintln!("Directory not found");
        return;
    }

    find_in_dir(dir, pattern);
}

