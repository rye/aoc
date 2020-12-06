pub fn binary_hone(instructions: &str, left: char, right: char) -> usize {
	let mut range: core::ops::Range<usize> = 0..2_usize.pow(instructions.len() as u32);

	for instruction in instructions.chars() {
		let midpoint = (range.start + range.end) / 2;

		match instruction {
			c if c == left => range = range.start..midpoint,
			c if c == right => range = midpoint..range.end,
			_ => panic!("strange instruction"),
		}
	}

	range.start
}

pub fn seat_id_from_bsp(bsp: &str) -> usize {
	let fb: &str = &bsp[0..7];
	let lr: &str = &bsp[7..10];

	let row: usize = binary_hone(fb, 'F', 'B');
	let column: usize = binary_hone(lr, 'L', 'R');

	row * 2_usize.pow(lr.len() as u32) + column
}

#[cfg(test)]
mod tests;
