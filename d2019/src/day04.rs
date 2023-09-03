pub type Intermediate = (u32, u32);
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let bounds: Vec<u32> = input
		.lines()
		.map(|line| -> Vec<u32> {
			line
				.split("-")
				.map(|num| -> u32 { num.parse::<u32>().expect("malformed bound") })
				.collect()
		})
		.flatten()
		.collect();

	assert_eq!(bounds.len(), 2);
	assert!(bounds[0] < bounds[1]);

	let starting_bound = bounds[0];
	let ending_bound = bounds[1];

	Ok((starting_bound, ending_bound))
}

#[must_use]
pub fn part_one((starting_bound, ending_bound): &Intermediate) -> Option<Output> {
	let valid_passwords = (*starting_bound..=*ending_bound)
		.filter(|number| {
			let password = Password(*number);
			password.is_monotonically_increasing() && password.has_at_least_two_adjacent_digits()
		})
		.collect::<Vec<u32>>();

	Some(valid_passwords.len())
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Password(u32);

#[derive(Debug, Copy, Clone)]
struct Digits<'i, I> {
	number: &'i I,
	pow: i32,
}

impl<'i> Password {
	fn digits(&'i self) -> Digits<'i, u32> {
		Digits {
			number: &self.0,
			pow: f64::from(self.0).log10() as i32,
		}
	}

	fn is_monotonically_increasing(&self) -> bool {
		let digits: Vec<u32> = self.digits().collect();
		let sorted: Vec<u32> = {
			let mut tmp = digits.clone();
			tmp.sort();
			tmp
		};
		sorted == digits
	}

	fn has_at_least_two_adjacent_digits(&self) -> bool {
		let mut digits: Vec<u32> = self.digits().collect();
		let starting_len = digits.len();
		digits.dedup();
		let final_len = digits.len();
		starting_len - final_len >= 1
	}
}

impl<'i> Iterator for Digits<'i, u32> {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		let number: &'i u32 = self.number;
		let power: i32 = self.pow;

		if self.pow >= 0 {
			let current: u32 = (number / 10_u32.pow(power as u32)) % 10_u32;
			self.pow -= 1;
			Some(current)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Password;

	#[test]
	fn digits_0() {
		let password: Password = Password(0_u32);
		let mut digits = password.digits();
		assert_eq!(digits.next(), None);
	}

	#[test]
	fn digits_1() {
		let password: Password = Password(1_u32);
		let mut digits = password.digits();
		assert_eq!(digits.next(), Some(1_u32));
		assert_eq!(digits.next(), None);
	}

	#[test]
	fn digits_123() {
		let password: Password = Password(123_u32);
		let mut digits = password.digits();
		assert_eq!(digits.next(), Some(1_u32));
		assert_eq!(digits.next(), Some(2_u32));
		assert_eq!(digits.next(), Some(3_u32));
		assert_eq!(digits.next(), None);
	}

	#[test]
	fn digits_123456() {
		let password: Password = Password(123456_u32);
		let mut digits = password.digits();
		assert_eq!(digits.next(), Some(1_u32));
		assert_eq!(digits.next(), Some(2_u32));
		assert_eq!(digits.next(), Some(3_u32));
		assert_eq!(digits.next(), Some(4_u32));
		assert_eq!(digits.next(), Some(5_u32));
		assert_eq!(digits.next(), Some(6_u32));
		assert_eq!(digits.next(), None);
	}
}
