pub enum Direction {
	Forward,
	Up,
	Down,
	None
}

type Movement = (Direction, i32);

pub fn parse(input: Vec<String>) -> Vec<Movement> {
	let mut items: Vec<Movement> = Vec::new();

	for line in input {
		let spl: Vec<&str> = line.split_whitespace().collect();

		if let (Some(ds), Some(ss)) = (spl.get(0), spl.get(1)) {
			let direction = match ds.to_owned() {
				"forward" => Direction::Forward,
				"up" => Direction::Up,
				"down" => Direction::Down,
				_ => Direction::None,
			};
			let steps = ss.parse::<i32>().unwrap();

			items.push((direction, steps));
		}
	}

	return items;
}

pub fn part1(input: &Vec<Movement>) -> i32 {
	let mut horizontal = 0;
	let mut depth = 0;

	for (direction, steps) in input {
		match direction {
			Direction::Up => depth -= steps,
			Direction::Down => depth += steps,
			Direction::Forward => horizontal += steps,
			Direction::None => {},
		}
	}

	return horizontal * depth;
}

pub fn part2(input: &Vec<Movement>) -> i32 {
	let mut aim = 0;
	let mut horizontal = 0;
	let mut depth = 0;

	for (direction, steps) in input {
		match direction {
			Direction::Up => aim -= steps,
			Direction::Down => aim += steps,
			Direction::Forward => {
				horizontal += steps;
				depth += aim * steps;
			},
			Direction::None => {},
		}
	}

	return horizontal * depth;
}

