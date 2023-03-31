use std::{env, io, iter};
use regex::Regex;

fn main() {
	// interpret the first argument as the regex to be matched
	let args: Vec<String> = env::args().collect();
	let (regex_string, after): (&str, bool) = match args.len() {
		2 => (&args[1], false),
		3 if args[1].as_str() == "-a" || args[1].as_str() == "--after" => (&args[2], true),
		_ => panic!("invalid arguments"),
	};
	let regex = Regex::new(regex_string).expect("regex should be legal");

	// get the byte indices of the matches
	let stdin: Vec<String> = io::stdin().lines().collect::<Result<_, _>>().expect("should get input");
	let matches: Vec<Option<usize>> =
		if after {
			stdin
				.iter()
				.map(|line| {
					regex
						.find(line)
						.map(|m| m.end())
				})
				.collect()
		} else {
			stdin
				.iter()
				.map(|line| {
					regex
						.find(line)
						.map(|m| m.start())
				})
				.collect()
		};

	// get the maximum offset of the regex across all strings
	// if there are no matches, just echo stdin
	let Some(alignment) = matches.iter().flatten().copied().max() else {
		for line in stdin {
			println!("{}", line);
		}
		return;
	};

	// output the lines of the input, with the regex match padded with spaces to align with all other matches
	for (line, match_index) in iter::zip(stdin.into_iter(), matches.into_iter()) {
		println!("{}", match match_index {
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
