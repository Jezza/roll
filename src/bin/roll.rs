use roll::evaluate_str;

fn main() {
	let mut args = std::env::args();
	if args.len() >= 2 {
		args.next();

		let argument: String = args.collect();
		let value = evaluate_str(&argument);
		println!("Output: {}", value);
	} else {
		repl();
		return
	};
}

fn repl() {
	use std::io;
	use std::io::prelude::*;

	let input = std::io::stdin();
	let mut input = input.lock();

	let mut buffer = String::new();

	loop {
		print!(">>> ");
		std::io::stdout().flush().unwrap();
		match input.read_line(&mut buffer) {
			Ok(_) => {
				let value = evaluate_str(&buffer);
				println!("Output: {}", value);
			},
			Err(e) => println!("{}", e),
		};

		buffer.clear();
	}
}

