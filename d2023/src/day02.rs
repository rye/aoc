use core::str::FromStr;

pub type Intermediate = Vec<Game>;
pub type Output = u32;

#[derive(Debug)]
pub struct Game {
	number: u32,
	handfuls: Vec<Handful>,
}

impl FromStr for Game {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (game_n, handfuls) = s.split_at(s.find(": ").expect("expected \": \""));

		let number = game_n
			.split(" ")
			.last()
			.expect("expected \" \" in game_n part")
			.parse()
			.expect("expected game number");

		let handfuls = handfuls[": ".len()..]
			.split("; ")
			.map(str::parse::<Handful>)
			.collect::<Result<Vec<Handful>, _>>()?;

		Ok(Game { number, handfuls })
	}
}

#[derive(Debug)]
pub struct Handful {
	red_count: usize,
	green_count: usize,
	blue_count: usize,
}

impl FromStr for Handful {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<Vec<&str>> = s
			.split(", ")
			.map(|part| part.split(" ").collect::<Vec<&str>>())
			.collect();

		let mut red_count: Option<usize> = None;
		let mut green_count: Option<usize> = None;
		let mut blue_count: Option<usize> = None;

		for part in parts {
			match part[1] {
				"red" => red_count = Some(part[0].parse().expect("expected to parse as number")),
				"green" => green_count = Some(part[0].parse().expect("expected to parse as number")),
				"blue" => blue_count = Some(part[0].parse().expect("expected to parse as number")),
				_ => {}
			}
		}

		Ok(Handful {
			red_count: red_count.unwrap_or(0),
			green_count: green_count.unwrap_or(0),
			blue_count: blue_count.unwrap_or(0),
		})
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	input.lines().map(str::parse).collect()
}

fn game_ids_not_exceeding_counts_with_replacement(
	games: &Vec<Game>,
	red_count: usize,
	green_count: usize,
	blue_count: usize,
) -> impl Iterator<Item = u32> + '_ {
	games
		.iter()
		.filter(move |game| {
			let mut game_ok: bool = true;

			for handful in &game.handfuls {
				if handful.red_count > red_count
					|| handful.blue_count > blue_count
					|| handful.green_count > green_count
				{
					game_ok = false;
					break;
				}
			}

			game_ok
		})
		.map(|game| game.number)
}

#[must_use]
pub fn part_one(games: &Intermediate) -> Option<Output> {
	Some(game_ids_not_exceeding_counts_with_replacement(games, 12, 13, 14).sum())
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day02-part1"),
	Some(8)
);

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
