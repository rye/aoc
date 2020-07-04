mod one {
	use core::borrow::Borrow;
	use core::ops::Mul;
	use core::ops::Add;

	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
	pub struct Vec2d(pub i32, pub i32);

	impl Vec2d {
		pub fn manhattan_distance(&self, other: &Self) -> i32 {
			(self.0 - other.0).abs() + (self.1 - other.1).abs()
		}
	}

	impl Mul<i32> for Vec2d {
		type Output = Vec2d;

		fn mul(self, n: i32) -> Self::Output {
			Vec2d(self.0 * n, self.1 * n)
		}
	}

	impl<T: Borrow<Vec2d>> Add<T> for Vec2d {
		type Output = Vec2d;

		fn add(self, vec: T) -> Self::Output {
			Vec2d(self.0 + vec.borrow().0, self.1 + vec.borrow().1)
		}
	}

	#[derive(Debug, PartialEq)]
	pub enum Direction {
		Up,
		Down,
		Left,
		Right,
	}

	impl From<Direction> for Vec2d {
		fn from(d: Direction) -> Vec2d {
			match d {
				Direction::Up => Vec2d(0, 1),
				Direction::Down => Vec2d(0, -1),
				Direction::Left => Vec2d(-1, 0),
				Direction::Right => Vec2d(1, 0),
			}
		}
	}

	impl TryFrom<char> for Direction {
		type Error = SegmentParseError;

		fn try_from(c: char) -> Result<Self, Self::Error> {
			match c {
				'U' => Ok(Direction::Up),
				'D' => Ok(Direction::Down),
				'L' => Ok(Direction::Left),
				'R' => Ok(Direction::Right),
				_ => Err(SegmentParseError::InvalidDirection),
			}
		}
	}

	#[cfg(test)]
	mod tests {}
}

mod two {
	#[cfg(test)]
	mod tests {}
}

fn main() {
	println!("Hello, world!");
}
