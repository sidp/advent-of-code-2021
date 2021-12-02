pub fn run(values: Vec<i32>) {
	let mut increased = 0;

	for (i, current) in values.iter().enumerate() {
		if i == 0 {
			continue;
		}

		match values.get(i - 1) {
			Some(prev) => {
				if current > prev {
					increased += 1;
				}
			},
			None => {}
		}
	}

	println!("Day 1, answer: {}", increased);
}
