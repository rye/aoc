use std::collections::VecDeque;

#[derive(Debug)]
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
		let current_opcode: Opcode = Opcode::from(self.inner[self.head]);

		match current_opcode {
			Opcode::Add => {
				let pos_a = self.inner[self.head + 1];
				let pos_b = self.inner[self.head + 2];
				let pos_out = self.inner[self.head + 3];
				self.inner[pos_out as usize] = self.inner[pos_a as usize] + self.inner[pos_b as usize];
				self.head += 4;
				Some(())
			}
			Opcode::Mul => {
				let pos_a = self.inner[self.head + 1];
				let pos_b = self.inner[self.head + 2];
				let pos_out = self.inner[self.head + 3];
				self.inner[pos_out as usize] = self.inner[pos_a as usize] * self.inner[pos_b as usize];
				self.head += 4;
				Some(())
			}

			Opcode::Input => {
				let pos_in = self.inner[self.head + 1];

				// If the user has supplied us with an input in the queue, use it.
				// Otherwise, prompt for an input.
				if self.interactive {
					use std::io::{stdin, stdout, Write};

					let mut input: String = String::new();
					print!("<= ");
					let _ = stdout().flush();

					stdin().read_line(&mut input).expect("invalid input");
					let input: i32 = input.parse::<i32>().expect("invalid input");
					self.inner[pos_in as usize] = input;
				} else {
					if let Some(input) = self.input.pop_front() {
						self.inner[pos_in as usize] = input;
					} else {
						panic!(
							"Attempted to take input in non-interactive mode without anything in input buffer"
						);
					}
				}

				self.head += 2;

				Some(())
			}

			Opcode::Output => {
				let pos_out = self.inner[self.head + 1];

				if self.interactive {
					println!("=> {}", self.inner[pos_out as usize]);
				} else {
					self.output.push_back(self.inner[pos_out as usize]);
				}

				self.head += 2;

				Some(())
			}

			Opcode::Halt => None,
		}
	}

	pub fn run(&mut self) -> &Vec<i32> {
		loop {
			if self.step().is_none() {
				break self.data();
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
	use super::Intcode;

	#[test]
	fn pgm_input_output() {
		let mut program: Intcode = Intcode::from(vec![3, 0, 4, 0, 99]);

		// First, we should be able to set the input.
		assert_eq!(program.data(), &vec![3, 0, 4, 0, 99]);
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
		assert_eq!(program.run(), &vec![2, 0, 0, 0, 99]);
	}

	#[test]
	fn pgm_5_b() {
		let mut program: Intcode = Intcode::from(vec![2, 3, 0, 3, 99]);
		assert_eq!(program.run(), &vec![2, 3, 0, 6, 99]);
	}

	#[test]
	fn pgm_6() {
		let mut program: Intcode = Intcode::from(vec![2, 4, 4, 5, 99, 0]);
		assert_eq!(program.run(), &vec![2, 4, 4, 5, 99, 9801]);
	}

	#[test]
	fn pgm_9() {
		let mut program: Intcode = Intcode::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
		assert_eq!(program.run(), &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
	}
}
