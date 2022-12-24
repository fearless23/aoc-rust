use std::fs::read_to_string;

pub fn get_lines(day: &str, file_type: &str) -> Vec<String> {
	let path = &format!("src/aoc2022/day{day}/{file_type}.txt");
	println!("File Path: {path}");
	let input = read_to_string(path).unwrap();
	input.lines().map(|i| i.to_string()).collect()
}
