mod aoc2022;
use aoc2022::day1;
use aoc2022::day2;
use aoc2022::day3;
use aoc2022::day4;

fn main() {
	run_solution("4.2")
}

fn run_solution(name: &str) {
	match name {
		"1.1" => day1::p1::solution(),
		"1.2" => day1::p2::solution(),
		"2.1" => day2::p1::solution(),
		"2.2" => day2::p2::solution(),
		"3.1" => day3::p1::solution(),
		"3.2" => day3::p2::solution(),
		"4.1" => day4::p1::solution(),
		"4.2" => day4::p2::solution(),
		_ => panic!("unknown day or solution"),
	}
}
