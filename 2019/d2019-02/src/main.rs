mod one {
	pub mod intcode {
		#[derive(Debug)]
		pub enum Opcode {
			Add,
			Mul,
			Halt,
		}

		impl From<i32> for Opcode {
			fn from(raw: i32) -> Opcode {
				use Opcode::*;
				match raw {
					1 => Add,
					2 => Mul,
					99 => Halt,
					_ => panic!(),
				}
			}
		}

		#[derive(Debug)]
		pub struct Intcode {
			inner: Vec<i32>,
			head: usize,
		}

		impl Intcode {
			pub fn new(inner: Vec<i32>, head: usize) -> Self {
				Self { inner, head }
			}

			pub fn from_data(inner: Vec<i32>) -> Self {
				Self {
					inner,
					head: 0_usize,
				}
			}

			pub fn step(&mut self) -> Option<()> {
				todo!()
			}

			pub fn run(&mut self) -> Vec<i32> {
				todo!()
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
			assert_eq!(program.run(), vec![2, 0, 0, 0, 99]);
		}

		#[test]
		fn pgm_5_b() {
			let mut program: Intcode = Intcode::from(vec![2, 3, 0, 3, 99]);
			assert_eq!(program.run(), vec![2, 3, 0, 6, 99]);
		}

		#[test]
		fn pgm_6() {
			let mut program: Intcode = Intcode::from(vec![2, 4, 4, 5, 99, 0]);
			assert_eq!(program.run(), vec![2, 4, 4, 5, 99, 9801]);
		}

		#[test]
		fn pgm_9() {
			let mut program: Intcode = Intcode::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
			assert_eq!(program.run(), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
		}
	}
}

mod two {}

fn main() {
	use std::io::BufRead;

	let program: Vec<i32> = std::io::stdin()
		.lock()
		.lines()
		.flat_map(|line: Result<String, std::io::Error>| -> Vec<i32> {
			line
				.unwrap()
				.split(',')
				.map(|n| n.parse::<i32>().unwrap())
				.collect()
		})
		.collect();
}
