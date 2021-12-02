pub enum Command {
	Forward(usize),
	Down(usize),
	Up(usize),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandParseError {
	#[error("empty line")]
	EmptyLine,
	#[error("invalid units")]
	ParseUnits(#[from] std::num::ParseIntError),
	#[error("invalid command")]
	InvalidCommand,
}

impl core::str::FromStr for Command {
	type Err = CommandParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split(' ');
		match (split.next(), split.next(), split.next()) {
			(Some("forward"), Some(units), None) => units
				.parse()
				.map(Command::Forward)
				.map_err(CommandParseError::from),
			(Some("down"), Some(units), None) => units
				.parse()
				.map(Command::Down)
				.map_err(CommandParseError::from),
			(Some("up"), Some(units), None) => units
				.parse()
				.map(Command::Up)
				.map_err(CommandParseError::from),
			(Some(_), _, _) => Err(CommandParseError::InvalidCommand),
			(None, _, _) => Err(CommandParseError::EmptyLine),
		}
	}
}

type Intermediate = Vec<Command>;

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.map(str::parse)
		.collect::<Result<Vec<_>, CommandParseError>>()
		.unwrap()
}

type Solution = usize;

pub fn part_one(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
