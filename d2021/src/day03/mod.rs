type Intermediate = Vec<[char; 12]>;

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.map(str::chars)
		.map(std::str::Chars::collect::<Vec<_>>)
		.map(<[char; 12]>::try_from)
		.map(Result::unwrap)
		.collect()
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

pub fn part_one(strings: &Intermediate) -> Option<Solution> {
	let statistics: [[usize; 2]; 12] = strings.iter().fold([[0; 2]; 12], bit_count);

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

fn find_o2_generator_rating(strings: &Vec<[char; 12]>) -> String {
	let mut idx = 0;

	let mut partial: Vec<char> = vec![];

	let mut strings: Vec<[char; 12]> = strings.clone();

	loop {
		let statistics = strings.iter().fold([[0; 2]; 12], bit_count);

		let keep_bit = if statistics[idx][0] <= statistics[idx][1] {
			'1'
		} else {
			'0'
		};

		partial.push(keep_bit);

		strings = strings
			.into_iter()
			.filter(|string| string[0..=idx] == partial[0..=idx])
			.collect();

		if strings.len() == 1 {
			break strings[0].iter().collect();
		} else {
			idx += 1;
		}
	}
}

fn find_co2_scrubber_rating(strings: &Vec<[char; 12]>) -> String {
	let mut idx = 0;

	let mut partial: Vec<char> = vec![];

	let mut strings: Vec<[char; 12]> = strings.clone();

	loop {
		let statistics = strings.iter().fold([[0; 2]; 12], bit_count);

		let keep_bit = if statistics[idx][0] > statistics[idx][1] {
			'1'
		} else {
			'0'
		};

		partial.push(keep_bit);

		strings = strings
			.into_iter()
			.filter(|string| string[0..=idx] == partial[0..=idx])
			.collect();

		if strings.len() == 1 {
			break strings[0].iter().collect();
		} else {
			idx += 1;
		}
	}
}

pub fn part_two(strings: &Intermediate) -> Option<Solution> {
	let oxygen_generator_rating_bits = find_o2_generator_rating(&strings);

	let oxygen_generator_rating: u32 = u32::from_str_radix(&oxygen_generator_rating_bits, 2).unwrap();

	let co2_scrubber_rating_bits = find_co2_scrubber_rating(&strings);

	let co2_scrubber_rating: u32 = u32::from_str_radix(&co2_scrubber_rating_bits, 2).unwrap();

	Some(oxygen_generator_rating * co2_scrubber_rating)
}
