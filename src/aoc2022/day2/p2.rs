pub fn solution() {
	let moves = get_moves();
	// println!("MOVES {:?}", moves);
	let score = calculate_score(moves);
	println!("SCORE {:?}", score);
}

fn get_moves() -> Vec<Vec<String>> {
	let input = std::fs::read_to_string("src/aoc2022/day2/input.txt").unwrap();
	let input = input
		.lines()
		.map(|i| i.split(' ').map(|j| j.to_string()).collect());
	input.collect::<Vec<Vec<String>>>()
}

fn my_shape_score(m: String) -> u32 {
	match m.as_str() {
		"X" => 0, // loose
		"Y" => 3, // draw
		"Z" => 6, // win
		_ => panic!("unknown move"),
	}
}

fn parse_move(m: Vec<String>) -> u32 {
	let rock: u32 = 1;
	let paper: u32 = 2;
	let scissors: u32 = 3;
	// A = Rock     = X  1  (wins if Z, loss if Y, draw if X)
	// B = Paper    = Y  2  (wins if X, loss if Z)
	// C = Scissors = Z  3  (wins if Y, loss if X)
	// AX=draw, AY=Win, AZ=loss
	// BX=loss, BY=draw, BZ=win
	// CX=win, BY=loss, CZ=draw
	let score = my_shape_score(m[1].clone());
	let win_score = match m.join("").as_str() {
		// loose
		"AX" => scissors,
		"BX" => rock,
		"CX" => paper,
		// draw
		"AY" => rock,
		"BY" => paper,
		"CY" => scissors,
		// win
		"AZ" => paper,
		"BZ" => scissors,
		"CZ" => rock,
		_ => panic!("unknown move"),
	};
	score + win_score
}

fn calculate_score(rounds: Vec<Vec<String>>) -> u32 {
	let mut sum = 0;
	for round in rounds {
		sum += parse_move(round)
	}
	sum
}
