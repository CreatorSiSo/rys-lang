use interpreter::Interpreter;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::Path;

mod interpreter;
mod lexer;
mod literal;
mod parser;
mod token;
use lexer::Lexer;
use parser::Parser;

fn run(input: String) {
	let mut lexer = Lexer::new();
	let mut parser = Parser::new();

	match lexer.scan(input) {
		Ok(tokens) => {
			println!("--- Lexer ---");
			tokens.iter().for_each(|token| match token.typ {
				token::TokenType::NewLine => println!("·"),
				_ => print!("{token} "),
			});

			println!("\n\n--- Parser ---");
			match parser.parse(tokens.to_vec()) {
				Ok(expressions) => {
					println!("{:#?}", expressions);

					println!("\n--- Interpreter ---");
					for expression in expressions {
						println!("{:?}", Interpreter::evaluate(expression));
					}
				}
				Err(errors) => errors.iter().for_each(|err| println!("{err}")),
			}
			println!()
		}
		Err(errors) => errors.iter().for_each(|err| println!("{:?}", err)),
	}

	println!()
}

fn run_file<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
	run(std::fs::read_to_string(path)?);
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
				run(line);
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

	match args.next() {
		Some(value) => match value.as_str() {
			"help" | "-h" | "--help" => println!("Usage: rys [path to script]"),
			path => run_file(path)?,
		},
		None => run_repl(),
	}

	Ok(())
}
