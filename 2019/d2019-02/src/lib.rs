use core::convert::{TryFrom, TryInto};

pub enum OpCode {
	Add = 1,
	Mul = 2,
	Halt = 99,
}

#[derive(Debug)]
pub enum OpCodeInterpretationError {
	InvalidOpCode(usize),
}

impl TryFrom<usize> for OpCode {
	type Error = OpCodeInterpretationError;

	fn try_from(us: usize) -> Result<Self, Self::Error> {
		match us {
			1 => Ok(OpCode::Add),
			2 => Ok(OpCode::Mul),
			99 => Ok(OpCode::Halt),
			_ => Err(OpCodeInterpretationError::InvalidOpCode(us)),
		}
	}
}

pub fn intcode_0(pgm: &[usize]) -> usize {
	let mut memory: Vec<usize> = pgm.to_vec();
	let mut idx = 0;

	loop {
		if idx >= memory.len() {
			break memory[0];
		}

		match memory[idx].try_into().unwrap() {
			OpCode::Add => {
				let a = memory[memory[idx + 1]];
				let b = memory[memory[idx + 2]];
				let output = memory[idx + 3];

				memory[output] = a + b;
				idx += 4;
			}
			OpCode::Mul => {
				let a = memory[memory[idx + 1]];
				let b = memory[memory[idx + 2]];
				let output = memory[idx + 3];

				memory[output] = a * b;
				idx += 4;
			}
			OpCode::Halt => break memory[0],
		}
	}
}
