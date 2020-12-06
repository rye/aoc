use std::{
	collections::{BTreeSet, HashMap},
	io::{stdin, BufRead},
};

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

use d2020::day01;

pub fn input_to_string() -> String {
	use std::io::Read;
	let mut input = String::new();
	stdin()
		.read_to_string(&mut input)
		.expect("failed to read input to string");

	input
}

pub fn solve_part<Intermediate, Solution, Parser, Solver>(
	when: (usize, usize),
	part: u8,
	input: &str,
	parse: Parser,
	solve: Solver,
) where
	Parser: Fn(&str) -> Intermediate,
	Solver: Fn(&Intermediate) -> Option<Solution>,
	Solution: core::fmt::Debug + PartialEq,
{
	let intermediate = parse(&input);

	let solution = solve(&intermediate);

	if let Some(solution) = solution {
		println!(
			"Year {}, Day {}, Part {}: {:?}",
			when.0, when.1, part, solution
		);
	} else {
		eprintln!(
			"Year {}, Day {}, Part {}: No solution!",
			when.0, when.1, part
		);
	}
}

fn main() {
	let app = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.arg(
			Arg::with_name("year")
				.short("y")
				.long("year")
				.takes_value(true)
				.required(true)
				.value_name("YEAR")
				.help("The year of the exercise"),
		)
		.arg(
			Arg::with_name("day")
				.short("d")
				.long("day")
				.takes_value(true)
				.required(true)
				.value_name("DAY")
				.help("The day of the exercise"),
		)
		.arg(
			Arg::with_name("part")
				.short("p")
				.long("part")
				.takes_value(true)
				.required(false)
				.value_name("PART")
				.help("The part. If not specified, both parts are run if available."),
		);

	let matches = app.get_matches();

	let input = input_to_string();

	match (
		matches.value_of("year"),
		matches.value_of("day"),
		matches.value_of("part"),
	) {
		(Some(y), Some(d), Some(p)) => {
			match (y.parse::<usize>(), d.parse::<usize>(), p.parse::<u8>()) {
				(Ok(year), Ok(day), Ok(part)) => match (year, day, part) {
					(2020, 01, 1) => solve_part((year, day), part, &input, day01::parse, day01::part_one),
					(_, _, _) => todo!(),
				},
				(_, _, _) => todo!(),
			}
		}
		(Some(y), Some(d), None) => match (y.parse::<usize>(), d.parse::<usize>()) {
			(Ok(year), Ok(day)) => match (year, day) {
				(2020, 01) => {
					solve_part((year, day), 1, &input, day01::parse, day01::part_one);
					solve_part((year, day), 2, &input, day01::parse, day01::part_two);
				}
				(_, _) => todo!(),
			},
			(_, _) => todo!(),
		},
		_ => unimplemented!(),
	}
}
