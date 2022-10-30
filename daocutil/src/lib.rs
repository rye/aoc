use std::num::ParseIntError;

pub type Solver = fn(&str) -> Result<(), Box<dyn std::error::Error>>;

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

fn parse_just_numbers(str: &str) -> Result<u32, ParseIntError> {
	str.matches(char::is_numeric).collect::<String>().parse()
}

pub fn parse_day_identifier(str: &str) -> Option<u32> {
	match (str.parse(), parse_just_numbers(str)) {
		(Ok(u32), _) => Some(u32),
		(Err(_), Ok(u32)) => Some(u32),
		(_, _) => None,
	}
}

#[macro_export]
macro_rules! generate_solver {
	($place:path, $fn_name:ident) => {
		fn $fn_name(data: &str) -> Result<(), Box<dyn std::error::Error>> {
			use $place::{parse, part_one, part_two, Intermediate};

			let intermediate: Intermediate = parse(data)?;

			if let Some(part_one) = part_one(&intermediate) {
				println!("Part One: {}", part_one);
			}

			if let Some(part_two) = part_two(&intermediate) {
				println!("Part Two: {}", part_two);
			}

			Ok(())
		}
	};

	($($n:literal => $place:path as $fn_name:ident),* $(,)?) => {
		{
			$(
				daocutil::generate_solver!($place, $fn_name);
			)*

			{
				let mut map: std::collections::HashMap<u32, daocutil::Solver> = std::collections::HashMap::new();

				$(
					map.insert($n, $fn_name);
				)*

				map
			}
		}
	};

	($place:path) => {
		fn main() -> Result<(), Box<dyn std::error::Error>> {
			let solvers = {
				use $place as base;

				daocutil::generate_solver![
					1_u32 => base::day01 as day01,
					2_u32 => base::day02 as day02,
					3_u32 => base::day03 as day03,
					4_u32 => base::day04 as day04,
					5_u32 => base::day05 as day05,
					6_u32 => base::day06 as day06,
					7_u32 => base::day07 as day07,
					8_u32 => base::day08 as day08,
					9_u32 => base::day09 as day09,
					10_u32 => base::day10 as day10,
					11_u32 => base::day11 as day11,
					12_u32 => base::day12 as day12,
					13_u32 => base::day13 as day13,
					14_u32 => base::day14 as day14,
					15_u32 => base::day15 as day15,
					16_u32 => base::day16 as day16,
					17_u32 => base::day17 as day17,
					18_u32 => base::day18 as day18,
					19_u32 => base::day19 as day19,
					20_u32 => base::day20 as day20,
					21_u32 => base::day21 as day21,
					22_u32 => base::day22 as day22,
					23_u32 => base::day23 as day23,
					24_u32 => base::day24 as day24,
					25_u32 => base::day25 as day25,
				]
			};

			let mut args = std::env::args();

			let _ = args.next();

			if let Some(ident) = args.next() {
				if let Some(ident) = daocutil::parse_day_identifier(&ident) {
					if let Some(handler) = solvers.get(&ident) {
						let data: String = match args.next() {
							Some(filename) => daocutil::string_from(std::fs::File::open(filename)?)?,
							None => daocutil::string_from(std::io::stdin())?,
						};

						handler(&data)?;
					} else {
						println!("Day has no handler: {}", ident);
					}
				} else {
					println!("Unknown day identifier: {}", ident);
				}
			}

			Ok(())
		}
	};
}
