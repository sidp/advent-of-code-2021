mod common;
mod day1;

fn main() {
    let values = common::read("src/day1/input.txt");

    day1::run(values);
}
