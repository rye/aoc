use std::{collections::BTreeMap, io::stdin};

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

use d2020::{day01, day02, day03, day04, day05};

pub fn input_to_string() -> String {
	use std::io::Read;
	let mut input = String::new();
	stdin()
		.read_to_string(&mut input)
		.expect("failed to read input to string");

	input
}

// pub fn solve_part<Intermediate, Solution, Parser, Solver>(
// 	when: (usize, usize),
// 	part: u8,
// 	input: &str,
// 	parse: Parser,
// 	solve: Solver,
// ) where
// 	Parser: Fn(&str) -> Intermediate,
// 	Solver: Fn(&Intermediate) -> Option<Solution>,
// 	Solution: core::fmt::Debug + PartialEq,
// {
// 	let intermediate = parse(&input);

// 	let solution = solve(&intermediate);

// 	if let Some(solution) = solution {
// 		println!(
// 			"Year {}, Day {}, Part {}: {:?}",
// 			when.0, when.1, part, solution
// 		);
// 	} else {
// 		eprintln!(
// 			"Year {}, Day {}, Part {}: No solution!",
// 			when.0, when.1, part
// 		);
// 	}
// }

trait Solve<Solution> {
	fn solve(&self, data: &str) -> Option<Solution>;
}

struct SolverPair<
	Intermediate,
	Solution: core::fmt::Debug,
	Parser: Fn(&str) -> Intermediate,
	Solver: Fn(&Intermediate) -> Option<Solution>,
>(Parser, Solver);

impl<Intermediate, Solution, Parser, Solver> Solve<String>
	for SolverPair<Intermediate, Solution, Parser, Solver>
where
	Solution: core::fmt::Debug,
	Parser: Fn(&str) -> Intermediate,
	Solver: Fn(&Intermediate) -> Option<Solution>,
{
	fn solve(&self, data: &str) -> Option<String> {
		(self.1)(&(self.0)(data)).map(|d| format!("{:?}", d))
	}
}

struct Executor {
	solvers: BTreeMap<(usize, usize), BTreeMap<u8, Box<dyn Solve<String>>>>,
}

impl Executor {
	fn new() -> Self {
		Self {
			solvers: BTreeMap::new(),
		}
	}

	fn add_solver_pair<I: 'static, S: 'static, P: 'static, So: 'static>(
		&mut self,
		year: usize,
		day: usize,
		part: u8,
		solver: SolverPair<I, S, P, So>,
	) where
		S: core::fmt::Debug,
		P: Fn(&str) -> I,
		So: Fn(&I) -> Option<S>,
	{
		if let Some(group) = self.solvers.get_mut(&(year, day)) {
			if let None = group.get(&part) {
				group.insert(part, Box::new(solver));
			}
		} else {
			self.solvers.insert((year, day), BTreeMap::new());
			self.add_solver_pair(year, day, part, solver);
		}
	}

	fn run_solvers(&self, year: usize, day: usize, input: &str) {
		if let Some(map) = self.solvers.get(&(year, day)) {
			for part in map.keys() {
				self.run_solver(year, day, *part, input);
			}
		} else {
			eprintln!("No known solver for Year: {}, Day: {}", year, day);
		}
	}

	fn run_solver(&self, year: usize, day: usize, part: u8, input: &str) {
		if let Some(map) = self.solvers.get(&(year, day)) {
			if let Some(solver) = map.get(&part) {
				if let Some(output) = solver.solve(input) {
					println!("Year {}, Day {}, Part {}: {}", year, day, part, output);
				} else {
					eprintln!("Year {}, Day {}, Part {}: No Solution", year, day, part);
				}
			} else {
				eprintln!(
					"No known solver for Part {} on Year: {}, Day: {}",
					part, year, day
				);
			}
		} else {
			eprintln!("No known solver for Year: {}, Day: {}", year, day);
		}
	}

	fn run(&self, matches: &clap::ArgMatches, input: String) {
		match (
			matches.value_of("year").map(|y| y.parse::<usize>()),
			matches.value_of("day").map(|y| y.parse::<usize>()),
			matches.value_of("part").map(|y| y.parse::<u8>()),
		) {
			(Some(Ok(year)), Some(Ok(day)), Some(Ok(part))) => self.run_solver(year, day, part, &input),
			(Some(Ok(year)), Some(Ok(day)), None) => self.run_solvers(year, day, &input),
			(_, _, _) => unimplemented!(),
		}
	}
}

macro_rules! s {
	($executor:ident, $year:literal-$day:literal#$part:literal, $intermediate:ty, $solution:ty, $parser:path, $solver:path) => {
		let solver = SolverPair($parser, $solver);
		$executor.add_solver_pair($year, $day, $part, solver);
	};

	($executor:ident, $year:literal-$day:literal#$part:literal, $parser:path, $solver:path) => {
		let solver = SolverPair($parser, $solver);
		$executor.add_solver_pair($year, $day, $part, solver);
	};
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

	let mut m = Executor::new();

	{
		use day01::{parse, part_one, part_two};
		s!(m, 2020-01#1, BTreeSet<i64>, i64, parse, part_one);
		s!(m, 2020-01#2, BTreeSet<i64>, i64, parse, part_two);
	}

	{
		use day02::{parse, part_one, part_two};
		s!(m, 2020-02#1, Vec<(Rule, String)>, usize, parse, part_one);
		s!(m, 2020-02#2, Vec<(Rule, String)>, usize, parse, part_two);
	}

	{
		use day03::{parse, part_one, part_two};
		s!(m, 2020-03#1, Vec<Vec<char>>, usize, parse, part_one);
		s!(m, 2020-03#2, Vec<Vec<char>>, usize, parse, part_two);
	}

	{
		use day04::{parse, part_one, part_two};
		s!(m, 2020-04#1, parse, part_one);
		s!(m, 2020-04#2, parse, part_two);
	}

	{
		use day05::{parse, part_one, part_two};
		s!(m, 2020-05#1, parse, part_one);
		s!(m, 2020-05#2, parse, part_two);
	}

	m.run(&matches, input);
}
