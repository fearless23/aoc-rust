use super::super::get_lines::get_lines;

pub fn solution() {
	let lines = parse_lines();
	println!("Count containing within {:?}", lines);
}

// stacks => 4*stacks - 1 = line_length
// stacks = line_length + 1 / 4
// [L] [T] [M] [Q] [L] [C]     [Z]
// 123456789
// indices:     2, 6, 10, 14
// stack num:   1, 2, 3,  4
// stack range: 0, 1, 2,  3
// 2 + 4*n
fn parse_lines() -> Vec<Vec<String>> {
	let lines = get_lines("src/aoc2022/day5/demo.txt");
	let stack_size = (lines[0].len() + 1) / 4;
	println!("{stack_size}");

	let mut stacks: Vec<Vec<String>> = Vec::new();
	for line in &lines {
		if line.is_empty() {
			break;
		}
		println!("{}", line);
		let mut boxes: Vec<String> = Vec::new();
		for i in 0..stack_size {
			let index = i * 4 + 2;
			println!("{index}");
			println!("{} {}", line, &line[index..index + 1]);
			let box_name = line[index..index + 1].to_string();
			boxes.push(box_name);
		}
		stacks.push(boxes);
	}
	stacks
}
