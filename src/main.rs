mod common;
mod day1;

fn main() {
    // DAY 1
    let day1input = common::read("src/day1/input.txt");
    let day1part1 = day1::part1(&day1input);
    let day1part2 = day1::part2(&day1input);
    assert_eq!(day1part1, 1292);
    assert_eq!(day1part2, 1262);
    print_day("1", day1part1, day1part2);
}

fn print_day(day: &str, part1: i32, part2: i32) {
    println!("DAY {}\nPart 1: {}\nPart 2: {}", day, part1, part2);
}
