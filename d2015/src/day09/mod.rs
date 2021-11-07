use std::collections::{hash_map::Entry, HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

type Intermediate = RouteDistances;

type Distance = usize;
type Place<'p> = &'p str;

type DistanceMap<'p> = HashMap<(Place<'p>, Place<'p>), Distance>;
type PlaceSet<'p> = HashSet<Place<'p>>;
type RouteDistances = Vec<Distance>;

const LINE_PARSE_RE: &str = r"^(?P<start>\w+) to (?P<end>\w+) = (?P<distance>\d+)$";

fn parse_line<'input, 'regex>(
	regex: &'regex Regex,
	line: &'input str,
) -> Option<(&'input str, &'input str, usize)> {
	regex
		.captures(line)
		.map(|captures| {
			(
				captures.name("start"),
				captures.name("end"),
				captures.name("distance"),
			)
		})
		.and_then(|(start, end, distance)| match (start, end, distance) {
			(Some(start), Some(end), Some(distance)) => {
				Some((start.as_str(), end.as_str(), distance.as_str()))
			}
			_ => None,
		})
		.and_then(|(start, end, distance)| match distance.parse() {
			Ok(distance) => Some((start, end, distance)),
			Err(_) => None,
		})
}

pub fn parse(input: &str) -> Intermediate {
	let regex: Regex = Regex::new(LINE_PARSE_RE).unwrap();

	// First, process all the lines down to a collection of each of the pieces.
	let lines: Vec<(&str, &str, usize)> = input
		.lines()
		.filter_map(|line| parse_line(&regex, line))
		.collect();

	// Then, build the distance/place map.  Store:
	//
	// - the list of all seen places,
	// - the individual distances between A and B and B and A.
	let mut distances: DistanceMap = DistanceMap::default();
	let mut places: PlaceSet = PlaceSet::default();

	for line in lines {
		let start = line.0;
		let end = line.1;

		places.insert(start);
		places.insert(end);

		let distance = line.2;

		let normal_key = (start, end);
		let reverse_key = (end, start);

		if let Entry::Vacant(normal_entry) = distances.entry(normal_key) {
			normal_entry.insert(distance);
		}

		if let Entry::Vacant(reverse_entry) = distances.entry(reverse_key) {
			reverse_entry.insert(distance);
		}
	}

	// Finally, permute all the places together and produce a mapping of routes to their total distance
	places
		.iter()
		.permutations(places.len())
		.map(|permutation| {
			permutation
				.windows(2)
				.filter_map(|window| {
					let key = (window[0].clone(), window[1].clone());
					distances.get(&key)
				})
				.sum()
		})
		.collect()
}

type Solution = Distance;

pub fn part_one(route_distances: &Intermediate) -> Option<Solution> {
	route_distances.iter().min().map(ToOwned::to_owned)
}

pub fn part_two(route_distances: &Intermediate) -> Option<Solution> {
	route_distances.iter().max().map(ToOwned::to_owned)
}
