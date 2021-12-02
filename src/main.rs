mod common;
mod day1;

fn main() {
    let day1input = common::read("src/day1/input.txt");

    let day1part1 = day1::part1(&day1input);
    let day1part2 = day1::part2(&day1input);
    println!("DAY 1\nPart 1: {}\nPart 2: {}", day1part1, day1part2);

}
