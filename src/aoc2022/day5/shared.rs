use super::super::get_lines::get_lines;

pub fn get_data(file_type: &str) -> (Vec<Vec<String>>, Vec<MoveData>) {
	let (lines, b, c) = check_lines(file_type);
	let stack_lines = &lines[0..c - 1];
	let moves_lines = &lines[c + 1..];
	let stacks = parse_stack_lines(stack_lines, b);
	let move_data = parse_moves_lines(moves_lines, b);
	(stacks, move_data)
}

fn check_lines(file_type: &str) -> (Vec<String>, usize, usize) {
	let path = &format!("src/aoc2022/day5/{file_type}.txt");
	let lines = get_lines(path);
	let num_of_stacks = (lines[0].len() + 1) / 4;
	let mut empty_line = 0;
	for (line_index, line) in lines.iter().enumerate() {
		if line.is_empty() {
			empty_line = line_index;
			break;
		}
	}
	(lines, num_of_stacks, empty_line)
}

fn parse_stack_lines(stack_lines: &[String], num_of_stacks: usize) -> Vec<Vec<String>> {
	let mut stacks: Vec<Vec<String>> = Vec::with_capacity(num_of_stacks);
	for _ in 0..num_of_stacks {
		stacks.push(Vec::new()); // while moving boxes a stack can grow to any size
	}
	// println!("Stack Starting --> {:?}", stacks);
	for line in stack_lines {
		for stack_index in 0..num_of_stacks {
			let index = stack_index * 4 + 1;
			let box_name = line[index..index + 1].trim().to_string();
			if !box_name.is_empty() {
				stacks[stack_index].insert(0, box_name)
			}
		}
	}
	stacks
}

#[derive(Debug)]
pub struct MoveData {
	pub count: u32,
	pub from: usize,
	pub to: usize,
}

fn parse_moves_lines(moves_lines: &[String], _num_of_stacks: usize) -> Vec<MoveData> {
	let mut moves_data: Vec<MoveData> = Vec::new();
	for move_txt in moves_lines {
		// move 1 from 2 to 1 => ["move","1","from","2","to","1"]
		let z: Vec<u32> = move_txt
			.split(' ')
			.filter_map(|i| match i.parse::<u32>() {
				Ok(i) => Some(i),
				Err(_) => None,
			})
			.collect::<Vec<u32>>();
		moves_data.push(MoveData {
			count: z[0],
			from: (z[1] - 1) as usize,
			to: (z[2] - 1) as usize,
		})
	}
	// println!("move_data --> {:?}", move_data);
	moves_data
}
