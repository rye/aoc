pub fn slope(map: &Vec<Vec<char>>, (dx, dy): (usize, usize)) -> usize {
	let mut position = (0, 0);
	let mut hits = 0;

	loop {
		let c: char = map[position.1][position.0];

		if c == '#' {
			hits += 1;
		}

		if position.1 < map.len() - 1 {
			position.0 += dx;
			position.1 += dy;

			if position.0 >= map[position.1].len() {
				position.0 %= map[position.1].len();
			}
		} else {
			break;
		}
	}

	hits
}

pub type Intermediate = Vec<Vec<char>>;
pub type Solution = usize;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(data.lines().map(|line| line.chars().collect()).collect())
}

pub fn part_one(map: &Intermediate) -> Option<Solution> {
	Some(slope(&map, (3, 1)))
}

pub fn part_two(map: &Intermediate) -> Option<Solution> {
	let trajectories = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	Some(
		trajectories
			.iter()
			.map(|trajectory| slope(&map, *trajectory))
			.fold(1, |acc, x| acc * x),
	)
}

#[cfg(test)]
mod tests;
