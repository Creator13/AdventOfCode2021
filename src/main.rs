fn main() {
    day1::run();
    day2::run();
}

mod day1 {
    use aoc2021::{challenge1, input};
    use std::time::Instant;

    pub fn run() {
        println!("Day 1: ");
        let input = input::read_and_parse("input/challenge1-1.txt").unwrap();

        print!("\t");
        challenge1(&input);

        print!("\t");
        challenge2(&input);
    }

    fn challenge1(input: &Vec<i32>) {
        let before = Instant::now();
        let result = challenge1::count_increments(input);

        println!("1-1: Result: {} (took {:.1?})", result, before.elapsed());
    }

    fn challenge2(input: &Vec<i32>) {
        let before = Instant::now();
        let result = challenge1::count_increments_windows(input);

        println!("1-2: Result: {} (took {:.1?})", result, before.elapsed());
    }
}

mod day2 {
    use aoc2021::{challenge2, input};
    use std::time::Instant;

    pub fn run() {
        let input = input::read("input/challenge2.txt").unwrap();

        println!("Day 2:");
        print!("\t");
        challenge1(&input);
    }

    fn challenge1(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge2::horizontal_pos_and_depth(&input);

        println!("2-1: Result: {} (took {:.1?})", result, before.elapsed());
    }
}
