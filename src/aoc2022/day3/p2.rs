use super::char_priority::get_char_priority;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn solution() {
	let rucksacks = get_lines();
	let score = get_score(rucksacks);
	println!("score {:?}", score);
}

fn get_lines() -> Vec<String> {
	let input = read_to_string("src/aoc2022/day3/input.txt").unwrap();
	input.lines().map(|i| i.to_string()).collect()
}

fn get_score(rucksacks: Vec<String>) -> u32 {
	let groups = rucksacks.len() / 3;
	let mut sum = 0;
	for i in 0..groups {
		let slice = &rucksacks[(i * 3)..((i + 1) * 3)];
		let common_char = find_common_char(
			slice[0].to_string(),
			slice[1].to_string(),
			slice[2].to_string(),
		);
		let priority = match common_char {
			Some(i) => get_char_priority(i),
			None => 0,
		};
		sum += priority;
	}
	sum
}

fn find_common_char(a: String, b: String, c: String) -> Option<String> {
	let set_a: HashSet<char> = a.chars().collect();
	let set_b: HashSet<char> = b.chars().collect();
	let set_c: HashSet<char> = c.chars().collect();

	let set_ab = set_a
		.intersection(&set_b)
		.map(|i| i.to_owned())
		.collect::<HashSet<char>>();
	let set_abc = set_ab
		.intersection(&set_c)
		.map(|i| i.to_owned())
		.collect::<HashSet<char>>();

	if set_abc.is_empty() {
		return None;
	}
	let v = set_abc.into_iter().collect::<Vec<char>>();
	Some(v[0].to_string())
}
