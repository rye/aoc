mod lines;
pub use lines::*;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day16;
pub mod day19;
pub mod day25;

pub fn string_from(mut r: impl std::io::Read) -> std::io::Result<String> {
	let mut s: String = String::new();
	r.read_to_string(&mut s)?;
	Ok(s)
}
