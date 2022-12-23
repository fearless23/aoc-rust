use super::shared::get_data;
use super::shared::MoveData;

pub fn solution() {
	let (a, b) = get_data("input");
	let stacks = execute_moves(a, b);
	println!("Answer - {stacks}");
}

fn execute_moves(mut stacks: Vec<Vec<String>>, moves_data: Vec<MoveData>) -> String {
	for move_data in moves_data {
		let MoveData { to, from, count } = move_data;
		let mut moved_block: Vec<String> = Vec::new();
		for _ in 0..count {
			let a = stacks[from].pop().unwrap();
			moved_block.insert(0, a);
		}
		for a in moved_block {
			stacks[to].push(a);
		}
	}
	let mut top_boxes = String::from("");
	for s in stacks {
		top_boxes.push_str(s.last().unwrap())
	}
	top_boxes
}
// take from stack --> take last item
// push to stack -> push at last
