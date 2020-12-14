use std::io::{stdin, Read};
use std::{collections::*, str::FromStr};

use regex::Regex;

use d2020::day08::*;

#[derive(Debug)]
enum Instruction {
	Acc(isize),
	Jmp(isize),
	Nop,
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, ()> {
		match &s[0..3] {
			"nop" => Ok(Instruction::Nop),
			"acc" => Ok(Instruction::Acc(
				s.split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
			)),
			"jmp" => Ok(Instruction::Jmp(
				s.split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
			)),
			_ => todo!(),
		}
	}
}

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let instructions: VecDeque<Instruction> = data.lines().map(|ln| ln.parse().unwrap()).collect();
	let mut visits: VecDeque<usize> = VecDeque::new();
	visits.resize(instructions.len(), 0_usize);

	{
		let mut accumulator = 0_isize;
		let mut head = 0_usize;

		loop {
			if visits[head] != 0_usize {
				break;
			} else {
				visits[head] += 1_usize;

				let instruction = &instructions[head];

				match instruction {
					Instruction::Acc(x) => {
						accumulator = accumulator + x;
						head += 1
					}
					Instruction::Jmp(ofs) => {
						head = (head as isize).checked_add(*ofs).unwrap() as usize;
					}
					Instruction::Nop => {
						head += 1;
					}
				}
			}
		}

		println!("Part One: {:?}", accumulator);
	}

	{
		println!("Part Two: {:?}", ());
	}
}
