#[macro_export]
macro_rules! day_solver {
	($place:path, $fn_name:ident) => {
		fn $fn_name(data: &str) -> Result<(), Box<dyn std::error::Error>> {
			use $place::{parse, part_one, part_two};

			let intermediate = parse(data)?;

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

type Solver = fn(&str) -> Result<(), Box<dyn std::error::Error>>;

macro_rules! day_solvers {
	($($place:path => $fn_name:ident $n:literal),* $(,)?) => {
		{
			$(day_solver!($place, $fn_name);)*

			let mut map: std::collections::HashMap<u32, Solver> = std::collections::HashMap::new();

			$(
				map.insert($n, $fn_name);
			)*

			map
		}
	}
}

fn string_from(mut read: impl std::io::Read) -> std::io::Result<String> {
	let mut buf: String = String::new();
	read.read_to_string(&mut buf)?;
	Ok(buf)
}

fn parse_numeric_ident(str: &str) -> Option<u32> {
	str
		.matches(char::is_numeric)
		.collect::<String>()
		.parse()
		.ok()
}

fn get_day_from_ident(ident: &str) -> Option<u32> {
	if let Some(u32) = ident.parse().ok() {
		Some(u32)
	} else if let Some(u32) = parse_numeric_ident(ident) {
		Some(u32)
	} else {
		None
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let solvers = day_solvers![
		d2022::day01 => day01 01,
		d2022::day02 => day02 02,
		d2022::day03 => day03 03,
		d2022::day04 => day04 04,
		d2022::day05 => day05 05,
		d2022::day06 => day06 06,
		d2022::day07 => day07 07,
		d2022::day08 => day08 08,
		d2022::day09 => day09 09,
		d2022::day10 => day10 10,
		d2022::day11 => day11 11,
		d2022::day12 => day12 12,
		d2022::day13 => day13 13,
		d2022::day14 => day14 14,
		d2022::day15 => day15 15,
		d2022::day16 => day16 16,
		d2022::day17 => day17 17,
		d2022::day18 => day18 18,
		d2022::day19 => day19 19,
		d2022::day20 => day20 20,
		d2022::day21 => day21 21,
		d2022::day22 => day22 22,
		d2022::day23 => day23 23,
		d2022::day24 => day24 24,
		d2022::day25 => day25 25,
	];

	let mut args = std::env::args();

	let _ = args.next();

	if let Some(ident) = args.next() {
		if let Some(ident) = get_day_from_ident(&ident) {
			if let Some(handler) = solvers.get(&ident) {
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
