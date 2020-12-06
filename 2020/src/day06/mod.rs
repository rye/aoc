use std::collections::BTreeSet;

pub type Answer = char;

pub fn people_in_group(group: &str) -> impl Iterator<Item = &str> {
	group.split_whitespace()
}

pub fn answers(person: &str) -> impl Iterator<Item = Answer> + '_ {
	person.chars().filter(|c| c.is_alphabetic())
}

pub fn intersect_all(items: impl Iterator<Item = BTreeSet<Answer>>) -> Option<BTreeSet<Answer>> {
	items.fold(
		None,
		|state: Option<BTreeSet<Answer>>, answers: BTreeSet<Answer>| {
			if let Some(state) = state {
				Some(state.intersection(&answers).copied().collect())
			} else {
				Some(answers)
			}
		},
	)
}

#[cfg(test)]
mod tests;
