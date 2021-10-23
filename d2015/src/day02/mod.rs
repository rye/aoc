use std::num::ParseIntError;

pub struct Gift {
	length: usize,
	width: usize,
	height: usize,
}

impl core::str::FromStr for Gift {
	type Err = ParseIntError;
	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let dimensions: Vec<usize> = line
			.split('x')
			.map(usize::from_str)
			.filter_map(Result::ok)
			.collect();

		let (length, width, height) = (dimensions[0], dimensions[1], dimensions[2]);

		Ok(Self {
			length,
			width,
			height,
		})
	}
}

impl Gift {
	fn side_areas(&self) -> [usize; 3] {
		[
			self.length * self.width,
			self.width * self.height,
			self.height * self.length,
		]
	}

	fn surface_area(&self) -> usize {
		let three_side_sum: usize = self.side_areas().into_iter().sum();
		2_usize * three_side_sum
	}

	fn area_of_smallest_side(&self) -> usize {
		self.side_areas().into_iter().min().unwrap()
	}

	fn wrapping_paper_required(&self) -> usize {
		self.surface_area() + self.area_of_smallest_side()
	}
}

type Intermediate = Vec<Gift>;

pub fn parse(input: &str) -> Intermediate {
	fn line_to_gift(line: &str) -> Gift {
		line.parse().unwrap()
	}

	input.lines().map(line_to_gift).collect()
}

type Solution = usize;

pub fn part_one(gifts: &Intermediate) -> Option<Solution> {
	Some(gifts.iter().map(Gift::wrapping_paper_required).sum())
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
