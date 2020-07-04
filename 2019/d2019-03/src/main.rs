mod one {
	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
	pub struct Vec2d(pub i32, pub i32);

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
