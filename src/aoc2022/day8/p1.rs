use super::super::get_lines::get_lines;

pub fn solution() {
	let lines = get_lines("8", "input");
	let (rows, columns) = analyze_lines(lines);
	let answer = analyze_matrix(rows, columns);
	println!("Answer - {:?}", answer);
}

fn analyze_lines(lines: Vec<String>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
	let mut matrix: Vec<Vec<u32>> = Vec::new();
	for line in lines {
		matrix.push(line.chars().map(|i| i.to_digit(10).unwrap_or(0)).collect());
	}

	let row_length = matrix[0].len();
	let mut matrix_rot = Vec::new();
	for i in 0..row_length {
		let mut column: Vec<u32> = Vec::new();
		for row in &matrix {
			column.push(row[i]);
		}
		matrix_rot.push(column)
	}
	(matrix, matrix_rot)
}

fn analyze_matrix(rows: Vec<Vec<u32>>, columns: Vec<Vec<u32>>) -> u32 {
	let row_length = columns.len() as u32;
	let column_length = rows.len() as u32;
	let visible_from_edges = (row_length * 2) + (column_length * 2) - 4;
	// also produce matrix for top to bottom
	let mut answer: u32 = 0;
	let row_index = rows.len() - 1;
	for i in 1..row_index {
		let z = analyze_row(&rows[i]);
		println!("Visible trees in row {i} -- {z}");
		answer += z
	}
	let column_index = columns.len() - 1;
	for i in 1..column_index {
		let z = analyze_row(&columns[i]);
		println!("Visible trees in column {i} -- {z}");
		answer += z;
	}
	answer + visible_from_edges
}

fn analyze_row(row: &Vec<u32>) -> u32 {
	// from left
	let n = row.len();
	let mut visible = 0;
	let mut highest = row[0];
	let mut highest_index = 0;
	for i in 1..n - 1 {
		if row[i] > highest {
			highest = row[i];
			highest_index = i;
			visible += 1;
		}
	}
	// from right
	highest = row[n - 1];
	for i in (highest_index + 1..n - 1).rev() {
		if row[i] > highest {
			highest = row[i];
			visible += 1;
		}
	}
	visible
}
