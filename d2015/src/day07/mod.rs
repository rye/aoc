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
enum Input {
	Wire(WireId),
	Signal(Signal),
	And(WireId, WireId),
	Or(WireId, WireId),
	LShift(WireId, u16),
	RShift(WireId, u16),
	Not(WireId),
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
			Input::Wire(a) => write!(f, "{} -> {}", a, out),
			Input::Signal(signal) => write!(f, "{} -> {}", signal, out),
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
				Ok(signal) => Input::Signal(signal),
				Err(_) => Input::Wire(lhs_split.get(0).unwrap().into()),
			},
			3 => {
				let a: WireId = lhs_split.get(0).unwrap().into();

				match *lhs_split.get(1).unwrap() {
					"AND" => Input::And(a, lhs_split.get(2).unwrap().into()),
					"OR" => Input::Or(a, lhs_split.get(2).unwrap().into()),
					"LSHIFT" => Input::LShift(a, lhs_split.get(2).unwrap().parse().unwrap()),
					"RSHIFT" => Input::RShift(a, lhs_split.get(2).unwrap().parse().unwrap()),
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

	use super::{Connection, Input};

	#[test]
	fn connect_123_x() {
		assert_eq!(
			Connection::from_str("123 -> x"),
			Ok(Connection {
				input: Input::Signal(123),
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
				input: Input::Signal(456),
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

pub fn part_one(_connections: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_connections: &Intermediate) -> Option<Solution> {
	None
}
