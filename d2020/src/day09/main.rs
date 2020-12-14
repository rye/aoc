use std::collections::*;
use std::io::{stdin, Read};

use d2020::day09::*;

fn sus(numbers: &Vec<isize>) -> Option<isize> {
	let mut seen: HashSet<isize> = HashSet::new();

	for number in &numbers[0..25] {
		seen.insert(*number);
	}

	for number in &numbers[25..] {
		let mut not_found = true;

		for prior in &seen {
			if seen.contains(&(number - prior)) {
				not_found = false;
				seen.insert(*number);
				break;
			}
		}

		if not_found {
			return Some(*number);
		}
	}

	None
}

fn find_weakness(numbers: &Vec<isize>, impostor: isize) -> Option<Vec<isize>> {
	for start_idx in 0..numbers.len() - 2 {
		for end_idx in (start_idx + 1)..(numbers.len() - 1) {
			let mut sum = 0;
			for x in &numbers[start_idx..end_idx + 1] {
				sum += x;
				if sum > impostor {
					break;
				}
			}

			if sum > impostor {
				break;
			} else if sum == impostor {
				let region = numbers[start_idx..end_idx + 1]
					.into_iter()
					.cloned()
					.collect();
				return Some(region);
			}
		}
	}

	None
}

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let numbers: Vec<isize> = data.lines().map(|s| s.parse().unwrap()).collect();

	let impostor = sus(&numbers).expect("super sus");

	{
		println!("Part One: {:?}", impostor);
	}

	{
		let mut result = None;
		if let Some(set) = find_weakness(&numbers, impostor) {
			if let (Some(max), Some(min)) = (set.iter().max(), set.iter().min()) {
				result = Some(max + min);
			}
		}
		let result = result.expect("ultra sus");

		println!("Part Two: {:?}", result);
	}
}
