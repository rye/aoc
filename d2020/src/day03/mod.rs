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

#[cfg(test)]
mod tests;
