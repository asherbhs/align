use std::{io, iter};

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Args {
	regex: String,
	#[arg(short, long)] after: bool,
	#[arg(short, long)] skip:  Option<usize>,
}

fn main() {
	let args = Args::parse();
	let regex = Regex::new(&args.regex).expect("regex should be legal");

	// get the byte indices of the matches
	let stdin: Vec<String> = io::stdin()
		.lines()
		.collect::<Result<_, _>>()
		.expect("should get input");
	let matches = stdin.iter().map(|line|
		regex.find_iter(line).nth(args.skip.unwrap_or(0))
	);
	let offsets: Vec<Option<usize>> =
		if args.after {
			matches.map(|om| om.map(|m| m.end())).collect()
		} else {
			matches.map(|om| om.map(|m| m.start())).collect()
		};

	// get the maximum offset of the regex across all strings
	// if there are no matches, just echo stdin
	let Some(alignment) = offsets
		.iter()
		.flatten()
		.copied()
		.max()
	else {
		for line in stdin {
			println!("{}", line);
		}
		return;
	};

	// output the lines of the input, with the regex match padded with spaces to align with all other matches
	for (line, offset) in iter::zip(stdin.into_iter(), offsets.into_iter()) {
		println!("{}", match offset {
			None => line,
			Some(i) => {
				let mut aligned = String::with_capacity(line.len() + alignment - i);
				aligned.push_str(&line[0..i]);
				for _ in 0..(alignment - i) {
					aligned.push(' ');
				}
				aligned.push_str(&line[i..line.len()]);
				aligned
			}
		});
	}
}
