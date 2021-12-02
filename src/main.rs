mod common;
mod day1;

fn main() {
    // DAY 1
    let day1_input = common::read("src/day1/input.txt");
    let day1_parsed_input = day1::parse(day1_input);
    let day1_part1 = day1::part1(&day1_parsed_input);
    let day1_part2 = day1::part2(&day1_parsed_input);
    assert_eq!(day1_part1, 1292);
    assert_eq!(day1_part2, 1262);
    print_day("1", day1_part1, day1_part2);
}

fn print_day(day: &str, part1: i32, part2: i32) {
    println!("DAY {}\nPart 1: {}\nPart 2: {}", day, part1, part2);
}
