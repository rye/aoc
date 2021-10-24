pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub fn string_from(mut r: impl std::io::Read) -> std::io::Result<String> {
	let mut s: String = String::new();
	r.read_to_string(&mut s)?;
	Ok(s)
}

#[macro_export]
macro_rules! day_solver {
	( $transform:expr, $part_one:expr, $part_two:expr ) => {
		fn main() -> Result<(), Box<dyn std::error::Error>> {
			use ::std::io::stdin;
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

#[macro_export]
macro_rules! day_solver_std {
	() => {
		d2015::day_solver_from!(self);
	};
}

#[macro_export]
macro_rules! day_solver_from {
	($place:path) => {
		use $place::{parse, part_one, part_two};

		d2015::day_solver!(
			|data| { parse(data) },
			|intermediate| { part_one(intermediate) },
			|intermediate| { part_two(intermediate) }
		);
	};

	($place:path, $parser:ident) => {
		use $place::{part_one, part_two, $parser};

		d2015::day_solver!(
			|data| { $parser(data) },
			|intermediate| { part_one(intermediate) },
			|intermediate| { part_two(intermediate) }
		);
	};

	($place:path, $parser:ident, $part_one:ident, $part_two:ident) => {
		use $place::{$parser, $part_one, $part_two};

		d2015::day_solver!(
			|data| { $parser(data) },
			|intermediate| { $part_one(intermediate) },
			|intermediate| { $part_two(intermediate) }
		);
	};
}
