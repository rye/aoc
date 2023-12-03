type Intermediate = String;

pub fn parse(input: &str) -> Intermediate {
	input.lines().map(str::to_string).collect()
}

const fn is_zero(byte: u8) -> bool {
	byte == 0_u8
}

const fn is_upper_nibble_zero(byte: u8) -> bool {
	(byte >> 4) == 0_u8
}

const fn leading5(data: [u8; 16]) -> bool {
	is_zero(data[0]) && is_zero(data[1]) && is_upper_nibble_zero(data[2])
}

const fn leading6(data: [u8; 16]) -> bool {
	is_zero(data[0]) && is_zero(data[1]) && is_zero(data[2])
}

type Solution = usize;

const RANGE: core::ops::Range<usize> = 1_usize..10_000_000_usize;

pub fn part_one(stub: &Intermediate) -> Option<Solution> {
	RANGE
		.map(|n| (n, md5::compute(format!("{stub}{n}"))))
		.find(|n| leading5((n.1).0))
		.map(|tuple| tuple.0)
}

pub fn part_two(stub: &Intermediate) -> Option<Solution> {
	RANGE
		.map(|n| (n, md5::compute(format!("{stub}{n}"))))
		.find(|n| leading6((n.1).0))
		.map(|tuple| tuple.0)
}
