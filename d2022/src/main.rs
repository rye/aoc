use std::{error::Error, num::ParseIntError};

type Solver = fn(&str) -> Result<(), Box<dyn Error>>;

macro_rules! day_solver {
	($place:path, $fn_name:ident) => {
		fn $fn_name(data: &str) -> Result<(), Box<dyn Error>> {
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
}

macro_rules! day_solvers {
	($($n:literal => $place:path as $fn_name:ident),* $(,)?) => {
		{
			$(
				day_solver!($place, $fn_name);
			)*

			phf::phf_map! {
				$($n => $fn_name),*
			}
		}
	};

	($place:path) => {
		{
			use $place as base;

			day_solvers![
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
		}
	};
}

fn string_from(mut read: impl std::io::Read) -> std::io::Result<String> {
	let mut buf: String = String::new();
	read.read_to_string(&mut buf)?;
	Ok(buf)
}

fn parse_numeric_ident(str: &str) -> Result<u32, ParseIntError> {
	str.matches(char::is_numeric).collect::<String>().parse()
}

fn get_day_from_ident(ident: &str) -> Option<u32> {
	match (ident.parse(), parse_numeric_ident(ident)) {
		(Ok(u32), _) => Some(u32),
		(Err(_), Ok(u32)) => Some(u32),
		(_, _) => None,
	}
}

static SOLVERS: phf::Map<u32, Solver> = day_solvers!(d2022);

fn main() -> Result<(), Box<dyn Error>> {
	let mut args = std::env::args();

	let _ = args.next();

	if let Some(ident) = args.next() {
		if let Some(ident) = get_day_from_ident(&ident) {
			if let Some(handler) = SOLVERS.get(&ident) {
				let data: String = match args.next() {
					Some(filename) => string_from(std::fs::File::open(filename)?)?,
					None => string_from(std::io::stdin())?,
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
