use crate::challenge1::challenge1_1;
use crate::challenge1::challenge1_2;

fn main() {
    challenge1_1();
    challenge1_2();
}

mod challenge1 {
    use aoc2021::{challenge1, input};

    pub fn challenge1_1() {
        let input = input::read_and_parse("input/challenge1-1.txt").unwrap();
        let result = challenge1::count_increments(&input);

        println!("1-1: Result: {}", result);
    }

    pub fn challenge1_2() {
        let input = input::read_and_parse("input/challenge1-1.txt").unwrap();
        let result = challenge1::count_increments_windows(&input);

        println!("1-2: Result: {}", result);
    }
}
