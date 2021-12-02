use std::fs;

pub fn read(path: &str) -> Vec<i32> {
	let contents = fs::read_to_string(path).expect("Something went wrong when reading the file.");

  	let lines: Vec<&str> = contents.split("\n").collect();

	let mut numbers: Vec<i32> = Vec::new();

	for string in lines {
		let val = string.parse::<i32>().unwrap();
		numbers.push(val);
	}

	return numbers;
}
