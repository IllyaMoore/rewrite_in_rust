use std::env;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file1> [file2] ...", args[0]);
        std::process::exit(1);
    }

    for filename in &args[1..] {
        if let Err(e) = touch(filename) {
            eprintln!("Error touching '{}': {}", filename, e);
        }
    }
}

fn touch(path: &str) -> io::Result<()> {
    let path = Path::new(path);

    if path.exists() {
        let file = OpenOptions::new()
            .write(true)
            .open(path)?;
        
        let now = SystemTime::now();
        file.set_modified(now)?;
    } else {
        File::create(path)?;
    }

    Ok(())
}
