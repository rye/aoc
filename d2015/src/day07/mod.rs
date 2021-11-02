use core::str::FromStr;

type Signal = u16;

#[derive(PartialEq, Debug)]
struct WireId(String);

impl<T> From<T> for WireId
where
	T: AsRef<str>,
{
	fn from(id: T) -> Self {
		Self(id.as_ref().to_string())
	}
}

impl core::fmt::Display for WireId {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(PartialEq, Debug)]
enum Source {
	Wire(WireId),
	Signal(Signal),
}

impl<T> From<T> for Source
where
	T: AsRef<str>,
{
	fn from(s: T) -> Self {
		match s.as_ref().parse::<Signal>() {
			Ok(signal) => Source::Signal(signal),
			Err(_) => Source::Wire(s.into()),
		}
	}
}

impl core::fmt::Display for Source {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Self::Wire(wire) => write!(f, "{}", wire),
			Self::Signal(signal) => write!(f, "{}", signal),
		}
	}
}

#[derive(PartialEq, Debug)]
enum Input {
	Source(Source),
	And(Source, Source),
	Or(Source, Source),
	LShift(Source, u16),
	RShift(Source, u16),
	Not(Source),
}

#[derive(PartialEq, Debug)]
pub struct Connection {
	input: Input,
	output: WireId,
}

impl core::fmt::Display for Connection {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let out = &self.output;

		match &self.input {
			Input::Source(source) => write!(f, "{} -> {}", source, out),
			Input::And(a, b) => write!(f, "{} AND {} -> {}", a, b, out),
			Input::Or(a, b) => write!(f, "{} OR {} -> {}", a, b, out),
			Input::LShift(a, b) => write!(f, "{} LSHIFT {} -> {}", a, b, out),
			Input::RShift(a, b) => write!(f, "{} RSHIFT {} -> {}", a, b, out),
			Input::Not(a) => write!(f, "NOT {} -> {}", a, out),
		}
	}
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
	#[error("invalid instruction")]
	InvalidInstruction,
	#[error("failed to parse signal")]
	SignalParse(#[from] core::num::ParseIntError),
}

impl FromStr for Connection {
	type Err = Error;

	fn from_str(specifier: &str) -> Result<Self, Self::Err> {
		let (lhs, rhs): (&str, &str) = {
			let split: Vec<_> = specifier.split(" -> ").collect();
			(
				split.get(0).ok_or(Error::InvalidInstruction)?,
				split.get(1).ok_or(Error::InvalidInstruction)?,
			)
		};

		let lhs_split: Vec<&str> = lhs.split(' ').collect();

		let input: Input = match lhs_split.len() {
			1 => match lhs_split.get(0).unwrap().parse::<Signal>() {
				Ok(signal) => Input::Source(Source::Signal(signal)),
				Err(_) => Input::Source(Source::Wire(lhs_split.get(0).unwrap().into())),
			},
			3 => {
				let a: Source = lhs_split.get(0).unwrap().into();
				let op = *lhs_split.get(1).unwrap();

				//let a: WireId = lhs_split.get(0).unwrap().into();

				match op {
					"AND" | "OR" => {
						let b: Source = lhs_split.get(2).unwrap().into();

						match op {
							"AND" => Input::And(a, b),
							"OR" => Input::Or(a, b),
							_ => unreachable!(),
						}
					}
					"LSHIFT" | "RSHIFT" => {
						let b: u16 = lhs_split.get(2).unwrap().parse().unwrap();

						match op {
							"LSHIFT" => Input::LShift(a, b),
							"RSHIFT" => Input::RShift(a, b),
							_ => unreachable!(),
						}
					}
					_ => unreachable!(),
				}
			}
			2 => match *lhs_split.get(0).unwrap() {
				"NOT" => Input::Not(lhs_split.get(1).unwrap().into()),
				_ => unreachable!(),
			},
			_ => unreachable!(),
		};

		let output: WireId = WireId(rhs.to_string());

		Ok(Connection { input, output })
	}
}

#[cfg(test)]
mod connection {
	use core::str::FromStr;

	use super::{Connection, Input, Source};

	#[test]
	fn connect_123_x() {
		assert_eq!(
			Connection::from_str("123 -> x"),
			Ok(Connection {
				input: Input::Source(Source::Signal(123)),
				output: "x".into()
			})
		)
	}

	#[test]
	fn connect_456_y() {
		let connection = "456 -> y";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::Source(Source::Signal(456)),
				output: "y".into(),
			})
		)
	}

	#[test]
	fn connect_xandy_d() {
		let connection = "x AND y -> d";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::And("x".into(), "y".into()),
				output: "d".into(),
			})
		)
	}
	#[test]
	fn connect_xory_e() {
		let connection = "x OR y -> e";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::Or("x".into(), "y".into()),
				output: "e".into(),
			})
		)
	}

	#[test]
	fn connect_xlshift2_f() {
		let connection = "x LSHIFT 2 -> f";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::LShift("x".into(), 2),
				output: "f".into(),
			})
		)
	}

	#[test]
	fn connect_yrshift2_g() {
		let connection = "y RSHIFT 2 -> g";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::RShift("y".into(), 2),
				output: "g".into(),
			})
		)
	}

	#[test]
	fn connect_notx_h() {
		let connection = "NOT x -> h";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::Not("x".into()),
				output: "h".into(),
			})
		)
	}

	#[test]
	fn connect_noty_i() {
		let connection = "NOT y -> i";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::Not("y".into()),
				output: "i".into(),
			})
		)
	}
}

type Intermediate = Vec<Connection>;

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.map(core::str::FromStr::from_str)
		.filter_map(Result::ok)
		.collect()
}

type Solution = Signal;

struct Connections<'c> {
	inputs: Vec<&'c Connection>,
}

pub fn part_one(connections: &Intermediate) -> Option<Solution> {
	let sources: Vec<_> = connections
		.iter()
		.filter(|input| match input.input {
			Input::Source(_) => true,
			_ => false,
		})
		.collect();

	for source in &sources {
		println!("{}", source);
	}

	// Something where I can...
	//
	// signals.set(WireId, SignalValue)

	None
}

pub fn part_two(_connections: &Intermediate) -> Option<Solution> {
	None
}
