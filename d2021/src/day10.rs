pub struct Chunk {
	open: char,
	contents: Vec<Chunk>,
}

fn chunkify(line: &str) -> Result<Vec<Chunk>, ChunkError> {
	todo!()
}

#[derive(Debug, thiserror::Error)]
enum ChunkError {
	#[error("Expected {1}, but found {0} instead.")]
	UnexpectedClose(char, char),
}

pub struct Subsystem {
	lines: Vec<Result<Vec<Chunk>, ChunkError>>,
}

impl core::str::FromStr for Subsystem {
	type Err = core::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lines = s.lines().map(chunkify).collect();
		Ok(Self { lines })
	}
}

pub type Intermediate = Subsystem;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	input.parse()
}

type Solution = u32;

pub fn part_one(_subsystem: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_subsystem: &Intermediate) -> Option<Solution> {
	None
}
