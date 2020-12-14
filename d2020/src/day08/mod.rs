use core::str::FromStr;
use std::collections::*;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	Acc(isize),
	Jmp(isize),
	Nop(isize),
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, ()> {
		let parameter: isize = s.split(' ').nth(1).unwrap().parse::<isize>().unwrap();

		match &s[0..3] {
			"nop" => Ok(Instruction::Nop(parameter)),
			"acc" => Ok(Instruction::Acc(parameter)),
			"jmp" => Ok(Instruction::Jmp(parameter)),
			_ => todo!(),
		}
	}
}

#[derive(Debug)]
pub enum ExecutionResult<T> {
	Normal(T),
	Looped(T),
}

impl<T> ExecutionResult<T> {
	pub fn unwrap(self) -> T {
		match self {
			Self::Normal(inner) => inner,
			Self::Looped(inner) => inner,
		}
	}
}

pub fn execute_program(program: &VecDeque<Instruction>) -> ExecutionResult<isize> {
	let mut visits: VecDeque<usize> = VecDeque::new();
	visits.resize(program.len(), 0_usize);

	let mut accumulator = 0_isize;
	let mut head = 0_usize;

	loop {
		if head == program.len() {
			break ExecutionResult::Normal(accumulator);
		} else if visits[head] != 0_usize {
			break ExecutionResult::Looped(accumulator);
		} else {
			visits[head] += 1_usize;

			let instruction = &program[head];

			match instruction {
				Instruction::Acc(x) => {
					accumulator = accumulator + x;
					head += 1
				}
				Instruction::Jmp(ofs) => {
					head = (head as isize).checked_add(*ofs).unwrap() as usize;
				}
				Instruction::Nop(_) => {
					head += 1;
				}
			}
		}
	}
}
