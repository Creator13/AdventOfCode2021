fn main() {
    day_1::run();
}

mod day_1 {
    use aoc2021::{challenge1, input};
    use std::time::Instant;

    pub fn run() {
        println!("Day 1: ");
        let input = input::read_and_parse("input/challenge1-1.txt").unwrap();

        print!("\t");
        challenge1_1(&input);

        print!("\t");
        challenge1_2(&input);
    }

    fn challenge1_1(input: &Vec<i32>) {
        let before = Instant::now();
        let result = challenge1::count_increments(input);

        println!("1-1: Result: {} (took {:.1?})", result, before.elapsed());
    }

    fn challenge1_2(input: &Vec<i32>) {
        let before = Instant::now();
        let result = challenge1::count_increments_windows(input);

        println!("1-2: Result: {} (took {:.1?})", result, before.elapsed());
    }
}
