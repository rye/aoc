use std::collections::*;
use std::io::{stdin, Read};

use d2020::day08::*;

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let instructions: VecDeque<Instruction> = data.lines().map(|ln| ln.parse().unwrap()).collect();

	{
		let accumulator = execute_program(&instructions).unwrap();
		println!("Part One: {:?}", accumulator);
	}

	{
		let instructions = instructions.clone();

		let mut accumulator = None;

		for position in 0..instructions.len() {
			if let Instruction::Acc(_x) = instructions[position] {
				continue;
			} else {
				let mut program = instructions.clone();

				match program[position] {
					Instruction::Jmp(ofs) => program[position] = Instruction::Nop(ofs),
					Instruction::Nop(par) => program[position] = Instruction::Jmp(par),
					Instruction::Acc(_) => unreachable!(),
				}

				let result = execute_program(&program);

				if let ExecutionResult::Normal(acc) = result {
					accumulator = Some(acc);
					break;
				} else {
					continue;
				}
			}
		}

		let accumulator = accumulator.expect("bruh");

		println!("Part Two: {:?}", accumulator);
	}
}
