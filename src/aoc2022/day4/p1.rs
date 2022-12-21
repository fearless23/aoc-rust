use super::super::get_lines::get_lines;

pub fn solution() {
	let lines = parse_lines();
	let count = analyze_lines(lines);
	println!("Count containing within {:?}", count);
}

fn parse_lines() -> Vec<Vec<u32>> {
	let lines = get_lines("src/aoc2022/day4/input.txt");
	let mut parsed_lines: Vec<Vec<u32>> = Vec::new();
	for line in lines {
		let parsed_line = line
			.split(',')
			.flat_map(|i| i.split('-'))
			.map(|i| i.parse::<u32>().unwrap())
			.collect::<Vec<u32>>();
		// let a = parsed_line[0..4];
		parsed_lines.push(parsed_line);
	}
	parsed_lines
}

fn analyze_lines(lines: Vec<Vec<u32>>) -> u32 {
	let mut count = 0;
	for line in lines {
		if analyze_line(line) {
			count += 1;
		}
	}
	count
}

fn analyze_line(line: Vec<u32>) -> bool {
	// let [a, b, c, d] = line[0..4];
	let (a, b, c, d) = if let [a, b, c, d] = line[0..4] {
		(a, b, c, d)
	} else {
		panic!("unknown line");
	};
	//   a...............................b
	//
	if c < a {
		d >= b
	} else if c == a {
		true
	} else if c < b {
		d <= b
	} else if c == b {
		d == b
	} else {
		false
	}
}
