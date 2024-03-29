mod aoc2022;
use aoc2022::day1;
use aoc2022::day2;
use aoc2022::day3;
use aoc2022::day4;
use aoc2022::day5;
use aoc2022::day6;
// use aoc2022::day7;
use aoc2022::day8;

fn main() {
	run_solution("8.1")
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
		"5.1" => day5::p1::solution(),
		"5.2" => day5::p2::solution(),
		"6.1" => day6::p1::solution(),
		"6.2" => day6::p2::solution(),
		// "7.1" => day7::p1::solution(),
		// "7.2" => day7::p2::solution(),
		"8.1" => day8::p1::solution(),
		"8.2" => day8::p2::solution(),
		_ => panic!("unknown day or solution"),
	}
}
