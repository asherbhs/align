use std::{env, io, iter};
use regex::Regex;

fn main() {
	let regex_string = &env::args().nth(1).expect("regex should be provided");
	let regex = Regex::new(regex_string).expect("regex should be legal");
	let stdin: Vec<String> = io::stdin().lines().collect::<Result<_, _>>().expect("should get input");
	let matches: Vec<Option<usize>> = stdin
		.iter()
		.map(|line| {
			regex
				.find(line)
				.map(|m| m.start())
		})
		.collect();
	let alignment: usize = matches.iter().flatten().copied().max().expect("need matching lines");
	for (line, match_index) in iter::zip(stdin.into_iter(), matches.into_iter()) {
		println!("{}", match match_index {
			None => line,
			Some(i) => {
				let mut aligned = String::with_capacity(line.len() + alignment - i);
				for (j, c) in line.chars().enumerate() {
					if j == i {
						for _ in 0..(alignment - i) {
							aligned.push(' ');
						}
					}
					aligned.push(c);
				}
				aligned
			}
		});
	}
}
