type Intermediate = (Vec<[char; 12]>, [[usize; 2]; 12]);

pub fn parse(input: &str) -> Intermediate {
	let lines: Vec<_> = input
		.lines()
		.map(str::chars)
		.map(std::str::Chars::collect::<Vec<_>>)
		.map(<[char; 12]>::try_from)
		.map(Result::unwrap)
		.collect();

	let statistics: [[usize; 2]; 12] = lines.iter().fold([[0; 2]; 12], bit_count);

	(lines, statistics)
}

// Each string in the input is 12 bits long. This means that they fit in a u16.  However,
// we need to be able to multiply without overflow.  (log(2^16 * 2^16) / log(2) == 32).
type Solution = u32;

fn bit_count(accumulator: [[usize; 2]; 12], string: &[char; 12]) -> [[usize; 2]; 12] {
	let mut accumulator = accumulator;

	for idx in 0..12 {
		match string[idx] {
			'0' => accumulator[idx][0] += 1,
			'1' => accumulator[idx][1] += 1,
			_ => unreachable!(),
		}
	}

	accumulator
}

pub fn part_one((_strings, statistics): &Intermediate) -> Option<Solution> {
	let gamma_rate_bits: String = statistics
		.iter()
		.map(|[zc, oc]| {
			if oc > zc {
				'1'
			} else if zc > oc {
				'0'
			} else {
				panic!("zc = {}, oc = {}, neither is more common!", zc, oc);
			}
		})
		.collect();

	let gamma_rate: u32 = u32::from_str_radix(&gamma_rate_bits, 2).unwrap();

	let epsilon_rate: u32 = !gamma_rate & 0b00001111_11111111;

	Some(gamma_rate * epsilon_rate)
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
