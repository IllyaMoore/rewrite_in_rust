use std::fs;
use std::path::Path;
use std::error::Error;
use std::process;

fn main() {
	if let Err(ref e) = run(Path::new(".")) {
		println!("{}", e);
		process::exit(1);
	}
}

fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				println!("{}", file_name);
		}
	}
	Ok(())
}


