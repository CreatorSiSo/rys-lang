use std::path::Path;
use std::process::exit;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn run_file<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	let _file_data = std::fs::read_to_string(path)?;
	Ok(())
}

fn run_repl() {
	let mut rl = Editor::<()>::new();
	if rl.load_history(".history").is_err() {
		println!("No previous history.");
	}
	loop {
		let readline = rl.readline(">> ");
		match readline {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				println!("Line: {}", line);
			}
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			}
			Err(ReadlineError::Eof) => {
				println!("CTRL-D");
				break;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		}
	}
	rl.save_history(".history").unwrap();
}

fn main() -> Result<(), std::io::Error> {
	let mut args = std::env::args().skip(1);
	if args.len() < 1 {
		println!("Usage: rys [path to script]");
		exit(64)
	}

	if let Some(arg) = args.next() {
		match arg.as_str() {
			"repl" => run_repl(),
			path => run_file(path)?,
		}
	}

	Ok(())
}
