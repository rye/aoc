use regex::Regex;

pub fn process_color(color: &str) -> String {
	color.replace(" ", "_")
}

pub fn process_content_spec(content_spec: &str) -> (usize, String) {
	let content_inner_spec = Regex::new("(\\d+) (.+) bags?").unwrap();

	let captures = content_inner_spec.captures(content_spec).unwrap();

	let number = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
	let color = process_color(captures.get(2).unwrap().as_str());

	(number, color)
}

pub fn process_contents(contents: &str) -> Vec<(usize, String)> {
	match contents {
		"no other bags" => Vec::new(),
		_ => contents
			.split(", ")
			.map(|content| process_content_spec(content))
			.collect(),
	}
}

pub fn duplicate_color(contents: &(usize, String)) -> Vec<String> {
	std::iter::repeat(contents.1.clone())
		.take(contents.0)
		.collect()
}

pub fn ruleify(container: String, contents: Vec<(usize, String)>) -> String {
	let colors: Vec<String> = contents
		.iter()
		.map(|tup| duplicate_color(tup))
		.flatten()
		.collect();

	format!("in({}, [{}]).", container, colors.join(", "))
}

pub type Intermediate = ();
type Solution = usize;

pub fn parse(_: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(())
}

pub fn part_one(_: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_: &Intermediate) -> Option<Solution> {
	None
}
