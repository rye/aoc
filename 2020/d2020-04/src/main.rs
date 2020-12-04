use std::io::{stdin, BufRead, Read};

fn main() {
	let data: String = {
		let mut string: String = String::new();
		stdin().read_to_string(&mut string);
		string
	};


	println!("Part One: {:?}", ());

	println!("Part Two: {:?}", ());
}

#[cfg(test)]
mod tests {}
