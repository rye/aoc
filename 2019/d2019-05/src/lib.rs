use core::convert::{TryFrom, TryInto};

pub struct Instruction {
	op: OpCode,
	modes: i32,
}

pub enum OpCode {
	Add = 01,
	Mul = 02,
	ReadInt = 03,
	PrintInt = 04,
	JumpIfTrue = 05,
	JumpIfFalse = 06,
	LessThan = 07,
	Equals = 08,
	Halt = 99,
}

#[derive(Debug)]
pub enum InterpretationError {
	InvalidParameterMode(i32),
	InvalidOpCode(i32),
}

impl TryFrom<i32> for OpCode {
	type Error = InterpretationError;

	fn try_from(us: i32) -> Result<Self, Self::Error> {
		match us {
			01 => Ok(OpCode::Add),
			02 => Ok(OpCode::Mul),
			03 => Ok(OpCode::ReadInt),
			04 => Ok(OpCode::PrintInt),
			05 => Ok(OpCode::JumpIfTrue),
			06 => Ok(OpCode::JumpIfFalse),
			07 => Ok(OpCode::LessThan),
			08 => Ok(OpCode::Equals),
			99 => Ok(OpCode::Halt),
			_ => Err(InterpretationError::InvalidOpCode(us)),
		}
	}
}

impl TryFrom<i32> for Instruction {
	type Error = InterpretationError;

	fn try_from(us: i32) -> Result<Self, Self::Error> {
		// The opcode is the last two digits
		//   A B C D E
		//         ^ ^
		let op: i32 = us % 100;
		let op: OpCode = op.try_into()?;

		let modes: i32 = us / 100;

		Ok(Instruction {
			op,
			modes
		})
	}
}

pub fn intcode_0(pgm: &[i32]) -> Vec<i32> {
	let mut memory: Vec<i32> = pgm.to_vec();
	let mut idx = 0;

	loop {
		if idx >= memory.len() {
			break memory;
		}

		let instruction: Instruction = memory[idx as usize].try_into().unwrap();

		match (instruction.op, (instruction.modes / 1) % 10, (instruction.modes / 10) % 10, (instruction.modes / 100) % 10) {
			(OpCode::Add, a_mode, b_mode, _) => {
				let [a, b] = match (a_mode, b_mode) {
					(0, 0) => [memory[memory[idx + 1] as usize], memory[memory[idx + 2] as usize]],
					(0, 1) => [memory[memory[idx + 1] as usize], memory[idx + 2]],
					(1, 0) => [memory[idx + 1], memory[memory[idx + 2] as usize]],
					(1, 1) => [memory[idx + 1], memory[idx + 2]],
					(_, _) => unimplemented!(),
				};

				// N.B. output needn't support immediate mode
				let output = memory[idx + 3 as usize];

				memory[output as usize] = a + b;
				idx += 4;
			}
			(OpCode::Mul, a_mode, b_mode, _) => {
				let [a, b] = match (a_mode, b_mode) {
					(0, 0) => [memory[memory[idx + 1] as usize], memory[memory[idx + 2] as usize]],
					(0, 1) => [memory[memory[idx + 1] as usize], memory[idx + 2]],
					(1, 0) => [memory[idx + 1], memory[memory[idx + 2] as usize]],
					(1, 1) => [memory[idx + 1], memory[idx + 2]],
					(_, _) => unimplemented!(),
				};

				let output = memory[idx + 3 as usize];

				memory[output as usize] = a * b;
				idx += 4;
			}
			(OpCode::ReadInt, _, _, _) => {
				let output = memory[idx + 1];

				let mut buffer = String::new();
				std::io::stdin().read_line(&mut buffer).expect("failed to read a line");
				let value: i32 = buffer.trim().parse().unwrap();

				memory[output as usize] = value;
				idx += 2;
			}
			(OpCode::PrintInt, a_mode, _, _) => {
				let a = match a_mode {
					0 => memory[memory[idx + 1] as usize],
					1 => memory[idx + 1],
					_ => unimplemented!(),
				};

				println!("{}: {}", idx, a);

				idx += 2;
			}
			(OpCode::JumpIfTrue, a_mode, b_mode, _) => {
				let [a, b] = match (a_mode, b_mode) {
					(0, 0) => [memory[memory[idx + 1] as usize], memory[memory[idx + 2] as usize]],
					(0, 1) => [memory[memory[idx + 1] as usize], memory[idx + 2]],
					(1, 0) => [memory[idx + 1], memory[memory[idx + 2] as usize]],
					(1, 1) => [memory[idx + 1], memory[idx + 2]],
					(_, _) => unimplemented!(),
				};

				match a {
					0 => idx += 3,
					_ => idx = b as usize,
				}
			}
			(OpCode::JumpIfFalse, a_mode, b_mode, _) => {
				let [a, b] = match (a_mode, b_mode) {
					(0, 0) => [memory[memory[idx + 1] as usize], memory[memory[idx + 2] as usize]],
					(0, 1) => [memory[memory[idx + 1] as usize], memory[idx + 2]],
					(1, 0) => [memory[idx + 1], memory[memory[idx + 2] as usize]],
					(1, 1) => [memory[idx + 1], memory[idx + 2]],
					(_, _) => unimplemented!(),
				};

				match a {
					0 => idx = b as usize,
					_ => idx += 3,
				}
			}
			(OpCode::LessThan, a_mode, b_mode, _) => {
				let [a, b] = match (a_mode, b_mode) {
					(0, 0) => [memory[memory[idx + 1] as usize], memory[memory[idx + 2] as usize]],
					(0, 1) => [memory[memory[idx + 1] as usize], memory[idx + 2]],
					(1, 0) => [memory[idx + 1], memory[memory[idx + 2] as usize]],
					(1, 1) => [memory[idx + 1], memory[idx + 2]],
					(_, _) => unimplemented!(),
				};

				let output: usize = memory[idx + 3] as usize;

				if a < b {
					memory[output] = 1;
				} else {
					memory[output] = 0;
				}

				idx += 4;
			}
			(OpCode::Equals, a_mode, b_mode, _) => {
				let [a, b] = match (a_mode, b_mode) {
					(0, 0) => [memory[memory[idx + 1] as usize], memory[memory[idx + 2] as usize]],
					(0, 1) => [memory[memory[idx + 1] as usize], memory[idx + 2]],
					(1, 0) => [memory[idx + 1], memory[memory[idx + 2] as usize]],
					(1, 1) => [memory[idx + 1], memory[idx + 2]],
					(_, _) => unimplemented!(),
				};

				let output: usize = memory[idx + 3] as usize;

				if a == b {
					memory[output] = 1;
				} else {
					memory[output] = 0;
				}

				idx += 4;
			}
			(OpCode::Halt, _, _, _) => break memory,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn modular_instructions() {
		let program: Vec<i32> = vec![1101,100,-1,4,0];
		let result: Vec<i32> = intcode_0(&program);
		assert_eq!(result[4], 99);

		let program: Vec<i32> = vec![1002, 4, 3, 4, 33];
		let result: Vec<i32> = intcode_0(&program);
		assert_eq!(result[4], 99);

		let program: Vec<i32> = vec![1101,100,-1,4,0];
		let result: Vec<i32> = intcode_0(&program);
		assert_eq!(result[4], 99);
	}
}
