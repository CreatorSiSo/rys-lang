use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::Path;

mod expr;
mod interpreter;
mod lexer;
mod literal;
mod parser;
mod token;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use token::TokenType;

fn run(input: String) {
	let mut lexer = Lexer::new();
	let mut parser = Parser::new();
	let mut interpreter = Interpreter::new();

	match lexer.scan(input) {
		Ok(tokens) => {
			println!("--- Lexer ---");
			tokens.iter().for_each(|token| match token.typ {
				TokenType::Semicolon => println!(";"),
				TokenType::NewLine => println!("·"),
				_ => print!("{token} "),
			});

			println!("\n\n--- Parser ---");
			match parser.parse(tokens.to_vec()) {
				Ok(ast) => {
					println!("{:#?}", ast);

					println!("\n--- Interpreter ---");
					if let Err(err) = interpreter.eval(ast) {
						println!("Runtime Error: {:?}", err)
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
