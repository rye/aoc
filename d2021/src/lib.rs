pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

/// Fully consumes a reader of type `std::io::Read` and produces a `String` containing all read text.
///
/// # Errors
///
/// An error is only returned if the underlying [`std::io::Read::read_to_string`] operation
/// returns an error.
///
/// See [`std::io::Read::read_to_string`] for all error semantics.
pub fn string_from(mut read: impl std::io::Read) -> std::io::Result<String> {
	let mut buf: String = String::new();
	read.read_to_string(&mut buf)?;
	Ok(buf)
}

/// Produces a "day solver" `main` definition.
///
/// # Generated Code
///
/// Arguments `$transform`, `$part_one`, and `$part_two` are all treated as expressions to be
/// evaluated against the underlying input after it has been processed.
///
#[macro_export]
macro_rules! day_solver {
	( $transform:expr, $part_one:expr, $part_two:expr ) => {
		fn main() -> Result<(), Box<dyn std::error::Error>> {
			use std::io::stdin;
			use $crate::string_from;

			let data: String = string_from(stdin())?;

			let intermediate = $transform(&data);

			if let Some(part_one) = $part_one(&intermediate) {
				println!("Part One: {}", part_one);
			}

			if let Some(part_two) = $part_two(&intermediate) {
				println!("Part Two: {}", part_two);
			}

			Ok(())
		}
	};
}

/// Generates a "day" solver from a given module.
///
/// # Examples
///
/// ```rust
/// mod dayXX {
/// 	type Intermediate = usize;
///
/// 	pub(super) fn parse(input: &str) -> Intermediate {
/// 		input.lines().count()
/// 	}
///
/// 	type Solution = usize;
///
/// 	pub(super) fn part_one(line_count: &Intermediate) -> Option<Solution> {
/// 		Some(*line_count)
/// 	}
///
/// 	pub(super) fn part_two(line_count: &Intermediate) -> Option<Solution> {
/// 		Some(*line_count)
/// 	}
/// }
///
/// d2021::day_solver_from!(dayXX);
/// ```
#[macro_export]
macro_rules! day_solver_from {
	($place:path) => {
		use $place::{parse, part_one, part_two};

		d2021::day_solver!(
			|data| { parse(data) },
			|intermediate| { part_one(intermediate) },
			|intermediate| { part_two(intermediate) }
		);
	};

	($place:path, $parser:ident) => {
		use $place::{part_one, part_two, $parser};

		d2021::day_solver!(
			|data| { $parser(data) },
			|intermediate| { part_one(intermediate) },
			|intermediate| { part_two(intermediate) }
		);
	};

	($place:path, $parser:ident, $part_one:ident, $part_two:ident) => {
		use $place::{$parser, $part_one, $part_two};

		d2021::day_solver!(
			|data| { $parser(data) },
			|intermediate| { $part_one(intermediate) },
			|intermediate| { $part_two(intermediate) }
		);
	};
}
