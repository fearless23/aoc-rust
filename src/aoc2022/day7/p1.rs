use super::super::get_lines::get_lines;

pub fn solution() {
	let lines = get_lines("7", "demo");
	let cd_commands = analyze_lines(lines);
	for cd in &cd_commands {
		println!("{:?}", cd);
	}
	let tree = construct_tree(cd_commands);
	print_folder(&tree, 1);
	// println!("Answer - {:?}", tree);
}

fn construct_tree(cd_commands: Vec<Vec<String>>) -> Folder {
	let root = listing_a_folder(cd_commands[0].to_owned());
	let rem = cd_commands[1..].to_owned();
	walk(root, rem)
}

fn walk(mut parent_folder: Folder, cd_commands: Vec<Vec<String>>) -> Folder {
	if cd_commands.is_empty() {
		return parent_folder;
	}
	let next_move = &cd_commands[0];
	let rem = cd_commands[1..].to_owned();
	let folder_name = get_folder_from_cd(&next_move[0]);
	if folder_name == ".." {
		walk(parent_folder, rem)
	} else {
		let folder = listing_a_folder(next_move.to_owned());
		let folder = walk(folder, rem);
		parent_folder.folders.push(folder);
		parent_folder
	}
}

fn listing_a_folder(data: Vec<String>) -> Folder {
	let folder_name = get_folder_from_cd(&data[0]);
	let folder_contents = data[2..].to_owned();
	let mut files: Vec<File> = Vec::new();
	let mut total_size = 0;
	for i in folder_contents {
		let j = i.split(' ').collect::<Vec<&str>>();
		let first = j[0];
		let last = j[1];
		// only fill folders when listed by cd command
		if first != "dir" {
			let size = first.parse::<u32>().unwrap();
			total_size += size;
			files.push(File {
				name: last.to_owned(),
				size,
			})
		}
	}

	Folder {
		name: folder_name,
		files,
		folders: Vec::new(),
		size: total_size,
	}
}

fn analyze_lines(lines: Vec<String>) -> Vec<Vec<String>> {
	let mut groups = Vec::new();
	let mut prev_group = false;
	let mut current_group: Vec<String> = Vec::new();
	for line in lines {
		if line.starts_with("$ cd") {
			if prev_group {
				groups.push(current_group.clone());
				current_group.clear();
			}
		} else {
			prev_group = true;
		}
		current_group.push(line.clone());
	}
	groups.push(current_group.clone());
	groups
}
#[derive(Debug)]
struct File {
	name: String,
	size: u32,
}

#[derive(Debug)]
struct Folder {
	name: String,
	size: u32,
	files: Vec<File>,
	folders: Vec<Folder>,
}

fn get_folder_from_cd(i: &str) -> String {
	i.split(' ').collect::<Vec<&str>>()[2].to_owned()
}

fn print_folder(folder: &Folder, level: u32) {
	println!("-{level}------------------------------");
	println!("----Folder {}", folder.name);
	for file in &folder.files {
		println!("----Folder {}", file.name);
	}
	for folder in &folder.folders {
		print_folder(folder, level + 1)
	}
}
