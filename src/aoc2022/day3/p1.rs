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
	let mut sum = 0;
	for rucksack in rucksacks {
		let (first, last) = rucksack.split_at(rucksack.len() / 2);
		let common_char = find_common_char(first.to_string(), last.to_string());
		let priority = match common_char {
			Some(i) => get_char_priority(i),
			None => 0,
		};
		sum += priority;
	}
	sum
}

fn find_common_char(a: String, b: String) -> Option<String> {
	let mut seta = HashSet::new();
	let mut setb = HashSet::new();
	for ca in a.chars() {
		seta.insert(ca);
	}
	for cb in b.chars() {
		setb.insert(cb);
	}
	let axx = seta.intersection(&setb).collect::<Vec<&char>>();
	if axx.is_empty() {
		return None;
	}
	Some(axx[0].to_string())
}

// collect is very good, can collect to any type
fn _get_lines_c() {
	let input = read_to_string("src/aoc2022/day3/input.txt").unwrap();
	let _out = input
		.lines()
		.map(|i| i.to_string().chars().collect::<HashSet<char>>())
		.collect::<Vec<HashSet<char>>>();
}
