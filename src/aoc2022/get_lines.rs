use std::fs::read_to_string;

pub fn get_lines(path: &str) -> Vec<String> {
	let input = read_to_string(path).unwrap();
	input.lines().map(|i| i.to_string()).collect()
}
