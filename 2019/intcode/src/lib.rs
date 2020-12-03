use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Opcode {
	Add,
	Mul,
	Input,
	Output,
	Halt,
}

impl From<i32> for Opcode {
	fn from(raw: i32) -> Opcode {
		use Opcode::*;

		// We should not get values >= 100.
		assert_eq!(raw % 100, raw);

		match raw {
			1 => Add,
			2 => Mul,
			3 => Input,
			4 => Output,
			99 => Halt,
			_ => panic!(),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum ParameterMode {
	Position,
	Immediate,
}

impl From<i32> for ParameterMode {
	fn from(raw: i32) -> ParameterMode {
		match raw {
			0 => Self::Position,
			1 => Self::Immediate,
			_ => unimplemented!(),
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
	opcode: Opcode,
	parameter_modes: (ParameterMode, ParameterMode, ParameterMode),
}

impl From<i32> for Instruction {
	fn from(raw: i32) -> Instruction {
		let opcode = Opcode::from(raw % 100);
		let mode_0: ParameterMode = ((raw / 100) % 10).into();
		let mode_1: ParameterMode = ((raw / 1000) % 10).into();
		let mode_2: ParameterMode = ((raw / 10000) % 10).into();
		let parameter_modes = (mode_0, mode_1, mode_2);

		Instruction {
			opcode,
			parameter_modes,
		}
	}
}

#[derive(Debug)]
pub struct Intcode {
	inner: Vec<i32>,
	head: usize,
	interactive: bool,
	input: VecDeque<i32>,
	output: VecDeque<i32>,
}

impl Intcode {
	pub fn new(inner: Vec<i32>, head: usize) -> Self {
		let (input, output) = (VecDeque::new(), VecDeque::new());

		Self {
			inner,
			head,
			interactive: true,
			input,
			output,
		}
	}

	pub fn from_data(inner: Vec<i32>) -> Self {
		Self::new(inner, 0_usize)
	}

	pub fn input(mut self, value: i32) -> Self {
		self.input.push_back(value);

		if self.interactive {
			self.interactive = false;
		}

		self
	}

	pub fn output(&mut self) -> Option<i32> {
		self.output.pop_front()
	}

	pub fn step(&mut self) -> Option<()> {
		let instruction: Instruction = Instruction::from(self.inner[self.head]);
		let opcode: Opcode = instruction.opcode;
		let parameter_modes: (ParameterMode, ParameterMode, ParameterMode) =
			instruction.parameter_modes;

		match opcode {
			Opcode::Add => {
				let param_a = self.inner[self.head + 1];
				let param_b = self.inner[self.head + 2];
				let param_outpos = self.inner[self.head + 3];

				// Ensure no illegal output parameter in immediate mode.
				assert!(parameter_modes.2 != ParameterMode::Immediate);

				let a = match parameter_modes.0 {
					ParameterMode::Position => self.inner[param_a as usize],
					ParameterMode::Immediate => param_a,
				};

				let b = match parameter_modes.1 {
					ParameterMode::Position => self.inner[param_b as usize],
					ParameterMode::Immediate => param_b,
				};

				self.inner[param_outpos as usize] = a + b;
				self.head += 4;
				Some(())
			}
			Opcode::Mul => {
				let param_a = self.inner[self.head + 1];
				let param_b = self.inner[self.head + 2];
				let param_outpos = self.inner[self.head + 3];

				// Ensure no illegal output parameter in immediate mode.
				assert!(parameter_modes.2 != ParameterMode::Immediate);

				let a = match parameter_modes.0 {
					ParameterMode::Position => self.inner[param_a as usize],
					ParameterMode::Immediate => param_a,
				};

				let b = match parameter_modes.1 {
					ParameterMode::Position => self.inner[param_b as usize],
					ParameterMode::Immediate => param_b,
				};

				self.inner[param_outpos as usize] = a * b;
				self.head += 4;
				Some(())
			}

			Opcode::Input => {
				let location = self.inner[self.head + 1];

				// If the user has supplied us with an input in the queue, use it.
				// Otherwise, prompt for an input.
				if self.interactive {
					use std::io::{stdin, stdout, Write};

					let mut input: String = String::new();
					print!("<= ");
					let _ = stdout().flush();

					stdin().read_line(&mut input).expect("invalid input");
					let input: i32 = input.parse::<i32>().expect("invalid input");
					self.inner[location as usize] = input;
				} else if let Some(input) = self.input.pop_front() {
					self.inner[location as usize] = input;
				} else {
					panic!(
						"Attempted to take input in non-interactive mode without anything in input buffer"
					);
				}

				self.head += 2;

				Some(())
			}

			Opcode::Output => {
				let param = self.inner[self.head + 1];

				let value = match parameter_modes.0 {
					ParameterMode::Position => self.inner[param as usize],
					ParameterMode::Immediate => param,
				};

				if self.interactive {
					println!("=> {}", value);
				} else {
					self.output.push_back(value);
				}

				self.head += 2;

				Some(())
			}

			Opcode::Halt => None,
		}
	}

	pub fn run(&mut self) -> &mut Self {
		loop {
			if self.step().is_none() {
				break self;
			}
		}
	}

	pub fn data(&self) -> &Vec<i32> {
		&self.inner
	}
}

impl From<Vec<i32>> for Intcode {
	fn from(program: Vec<i32>) -> Self {
		Self::from_data(program)
	}
}

#[cfg(test)]
mod tests {
	use super::{Instruction, Intcode};

	#[cfg(test)]
	mod instruction {
		use super::super::{Opcode, ParameterMode};
		use super::Instruction;

		#[test]
		fn standard_add_correct() {
			assert_eq!(
				Instruction::from(1),
				Instruction {
					opcode: Opcode::Add,
					parameter_modes: (
						ParameterMode::Position,
						ParameterMode::Position,
						ParameterMode::Position
					)
				}
			)
		}

		#[test]
		fn standard_mul_correct() {
			assert_eq!(
				Instruction::from(2),
				Instruction {
					opcode: Opcode::Mul,
					parameter_modes: (
						ParameterMode::Position,
						ParameterMode::Position,
						ParameterMode::Position
					)
				}
			)
		}

		#[test]
		fn standard_input_correct() {
			assert_eq!(
				Instruction::from(3),
				Instruction {
					opcode: Opcode::Input,
					parameter_modes: (
						ParameterMode::Position,
						ParameterMode::Position,
						ParameterMode::Position
					)
				}
			)
		}

		#[test]
		fn standard_output_correct() {
			assert_eq!(
				Instruction::from(4),
				Instruction {
					opcode: Opcode::Output,
					parameter_modes: (
						ParameterMode::Position,
						ParameterMode::Position,
						ParameterMode::Position
					)
				}
			)
		}
	}

	#[test]
	fn pgm_input_output() {
		let mut program: Intcode = Intcode::from(vec![3, 0, 4, 0, 99]);

		// First, we should be able to set the input.
		program = program.input(573);
		assert_eq!(program.data(), &vec![3, 0, 4, 0, 99]);

		// After one step, the value we input should be stored in the data, and we
		// shouldn't have any output.
		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![573, 0, 4, 0, 99]);
		assert_eq!(program.output(), None);

		// Stepping once more, we should now have an output.
		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![573, 0, 4, 0, 99]);
		assert_eq!(program.output(), Some(573));

		// Stepping once again should halt the program.
		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_add_immediate() {
		let mut program: Intcode = Intcode::from(vec![1101, 100, -1, 4, 0]);

		assert_eq!(program.data(), &vec![1101, 100, -1, 4, 0]);

		// Stepping, opcode 1101 should add the 100 and -1 and store the result, 99,
		// in position 4.
		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![1101, 100, -1, 4, 99]);

		// Next step should terminate the program.
		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_multiply_immediate() {
		let mut program: Intcode = Intcode::from(vec![1002, 4, 3, 4, 33]);

		assert_eq!(program.data(), &vec![1002, 4, 3, 4, 33]);

		// Stepping, opcode 1002 should multiply the value at address 4
		// by the literal value 3 and store it in position 4, so it effectively
		// multiplies the last position by 3, setting it to the halt instruction.
		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![1002, 4, 3, 4, 99]);

		// Next step should terminate the program.
		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_12() {
		let mut program: Intcode = Intcode::from(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
		assert_eq!(
			program.data(),
			&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
		);
		program.step();
		assert_eq!(
			program.data(),
			&vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
		);
		program.step();
		assert_eq!(
			program.data(),
			&vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
		);
	}

	#[test]
	fn pgm_5_a() {
		let mut program: Intcode = Intcode::from(vec![1, 0, 0, 0, 99]);
		assert_eq!(program.run().data(), &vec![2, 0, 0, 0, 99]);
	}

	#[test]
	fn pgm_5_b() {
		let mut program: Intcode = Intcode::from(vec![2, 3, 0, 3, 99]);
		assert_eq!(program.run().data(), &vec![2, 3, 0, 6, 99]);
	}

	#[test]
	fn pgm_6() {
		let mut program: Intcode = Intcode::from(vec![2, 4, 4, 5, 99, 0]);
		assert_eq!(program.run().data(), &vec![2, 4, 4, 5, 99, 9801]);
	}

	#[test]
	fn pgm_9() {
		let mut program: Intcode = Intcode::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
		assert_eq!(program.run().data(), &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
	}
}
