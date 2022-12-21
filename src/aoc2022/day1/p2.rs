pub fn solution() {
	let numbers = get_numbers_empty_as_zero();
	let sum = top_three_groups(numbers);
	println!("Sum of top three groups is {:?}", sum);
}

fn get_numbers_empty_as_zero() -> std::vec::Vec<u64> {
	let input = std::fs::read_to_string("src/read_file/input.txt").unwrap();
	let input = input.lines().map(|i| i.parse::<u64>().unwrap_or(0));
	input.collect::<Vec<u64>>()
}

fn top_three_groups(numbers: Vec<u64>) -> u64 {
	let mut groups: Vec<u64> = Vec::new();
	let mut sum: u64 = 0;
	for num in numbers {
		if num == 0 {
			groups.push(sum);
			sum = 0;
		} else {
			sum += num;
		}
	}

	let mut s: u64 = groups[0];
	let mut m: u64 = groups[1];
	let mut l: u64 = groups[2];

	for num in groups {
		// summed the group
		if num > s {
			if num <= m {
				// smallest
				s = num;
			} else {
				s = m;
				if num <= l {
					// new middle
					m = num;
				} else {
					// new largest
					m = l;
					l = num;
				}
			}
		}
	}
	s + m + l
}
