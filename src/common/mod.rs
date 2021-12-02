use std::fs;

pub fn read(path: &str) -> Vec<String> {
	let contents = fs::read_to_string(path).expect("Something went wrong when reading the file.");

  	let lines: Vec<&str> = contents.split("\n").collect();

	return lines.into_iter().map(|x| x.to_string()).collect();
}
