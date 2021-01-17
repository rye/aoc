use d2020::day02::*;

type Intermediate = Vec<(Rule, String)>;
type Solution = usize;

fn parse(data: &str) -> Intermediate {
	data
		.lines()
		.map(|s| {
			let rule = (&s).split(": ").nth(0).unwrap().parse::<Rule>().unwrap();
			let password = (&s).split(": ").nth(1).unwrap().to_string();

			(rule, password.clone())
		})
		.collect()
}

fn part_one(rules: &Intermediate) -> Option<Solution> {
	Some(
		rules
			.iter()
			.filter(|(rule, password)| validate_password(rule, password))
			.count(),
	)
}

fn part_two(rules: &Intermediate) -> Option<Solution> {
	Some(
		rules
			.iter()
			.filter(|(rule, password)| validate_password_two(rule, password))
			.count(),
	)
}

d2020::day_solver!(Intermediate, Solution, parse, part_one, part_two);
