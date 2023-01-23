use std::collections::HashSet;

use super::super::get_lines::get_lines;

pub fn solution() {
	let lines = get_lines("6", "input");
	let a = lines[0].as_str();
	let first = analyze_signal(a);
	println!("Answer - {}", first);
}

fn analyze_signal(a: &str) -> u32 {
	let li = a.len() - 13;
	for i in 0..li {
		let signal = &a[i..i + 14];
		if all_different(signal) {
			return (i + 14) as u32;
		}
	}
	100000000
}

fn all_different(a: &str) -> bool {
	let mut b: HashSet<char> = HashSet::new();
	for char in a.chars() {
		if b.contains(&char) {
			return false;
		} else {
			b.insert(char);
		}
	}
	true
}
