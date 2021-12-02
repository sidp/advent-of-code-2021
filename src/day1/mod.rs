pub fn parse(input: Vec<String>) -> Vec<i32> {
	let mut numbers: Vec<i32> = Vec::new();

	for line in input {
		let val = line.parse::<i32>().unwrap();
		numbers.push(val);
	}

	return numbers;
}

pub fn part1(values: &Vec<i32>) -> i32 {
	let mut increased = 0;

	for (i, current) in values.iter().enumerate() {
		if i == 0 {
			continue;
		}

		if let Some(prev) = values.get(i - 1) {
			if current > prev {
				increased += 1;
			}
		}
	}

	return increased;
}

pub fn part2(values: &Vec<i32>) -> i32 {
	let mut increased = 0;
	let mut i = 0;
	loop {
		if let (Some(previous), Some(current)) = (sum_window(values, i, 3), sum_window(values, i + 1, 3)) {
			if previous < current {
				increased += 1;
			}
		} else {
			break;
		}

		i += 1;
	}

	return increased;
}

fn sum_window(values: &Vec<i32>, index: usize, size: usize) -> Option<i32> {
	if let Some(slice) = values.get(index..index+size) {
		return Some(slice.iter().sum());
	} else {
		return None;
	}
}
