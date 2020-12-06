pub fn slope(data: &Vec<Vec<char>>, (dx, dy): (usize, usize)) -> usize {
	let mut position = (0, 0);
	let mut hits = 0;

	loop {
		let c: char = data[position.1][position.0];

		if c == '#' {
			hits += 1;
		}

		if position.1 < data.len() - 1 {
			position.0 += dx;
			position.1 += dy;

			if position.0 >= data[position.1].len() {
				position.0 %= data[position.1].len();
			}
		} else {
			break;
		}
	}

	hits
}

type Intermediate = Vec<Vec<char>>;
type Solution = usize;

pub fn parse(s: &str) -> Intermediate {
	s.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(map: &Intermediate) -> Option<Solution> {
	Some(slope(map, (3, 1)))
}

pub fn part_two(map: &Intermediate) -> Option<Solution> {
	Some(
		vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
			.drain(..)
			.map(|trajectory| slope(&map, trajectory))
			.fold(1, |acc, x| acc * x),
	)
}

#[cfg(test)]
mod tests;
