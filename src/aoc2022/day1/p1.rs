pub fn solution() {
	let numbers = get_numbers_empty_as_zero();
	let sum = get_max_group_sum(numbers);
	println!("Max is {:?}", sum);
}

fn _get_numbers() -> std::vec::Vec<u64> {
	let input = std::fs::read_to_string("src/read_file/input.txt").unwrap();
	let input = input.lines().filter_map(|i| match i.parse::<u64>() {
		Ok(i) => Some(i),
		Err(_) => None,
	});
	input.collect::<Vec<u64>>()
}

fn get_numbers_empty_as_zero() -> std::vec::Vec<u64> {
	let input = std::fs::read_to_string("src/read_file/input.txt").unwrap();
	let input = input.lines().map(|i| i.parse::<u64>().unwrap_or(0));
	input.collect::<Vec<u64>>()
}

fn get_max_group_sum(numbers: Vec<u64>) -> u64 {
	// let groups: Vec<u64> = Vec::new();
	let mut max_group_sum = 0;
	let mut sum: u64 = 0;
	for num in numbers {
		if num == 0 {
			if sum > max_group_sum {
				max_group_sum = sum;
			}
			sum = 0;
		} else {
			sum += num;
		}
	}
	max_group_sum
}
