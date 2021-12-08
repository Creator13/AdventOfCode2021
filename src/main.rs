fn main() {
    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
    day6::run();
    day7::run();
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
        print!("\t");
        challenge2(&input);
    }

    fn challenge1(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge2::move_directly(&input);

        println!("2-1: Result: {} (took {:.1?})", result, before.elapsed());
    }

    fn challenge2(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge2::move_by_aim(&input);

        println!("2-2: Result: {} (took {:.1?})", result, before.elapsed());
    }
}

mod day3 {
    use aoc2021::{challenge3, input};
    use std::time::Instant;

    pub fn run() {
        let input = input::read("input/challenge3.txt").unwrap();

        println!("Day 3:");
        print!("\t");
        challenge1(&input);
        print!("\t");
        challenge1_logical(&input);
        print!("\t");
        challenge2(&input);
    }

    fn challenge1(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge3::get_power_rates_binary(&input);

        println!("3-1 (binary): Result: {} (took {:.1?})", result.power_consumption(), before.elapsed());
    }

    fn challenge1_logical(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge3::get_power_rates_logical(&input);

        println!("3-1 (logical): Result: {} (took {:.1?})", result.power_consumption(), before.elapsed());
    }

    fn challenge2(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge3::get_life_rates(&input);

        println!("3-2: Result: {} (took {:.1?})", result.life_rating(), before.elapsed());
    }
}

mod day4 {
    use aoc2021::{challenge4, input};
    use std::time::Instant;

    pub fn run() {
        let input = input::read("input/challenge4.txt").unwrap();

        println!("Day 4:");
        print!("\t");
        challenge1(&input);
        print!("\t");
        challenge2(&input);
    }

    fn challenge1(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge4::challenge1(&input);

        println!("4-1: Result: {} (took {:.1?})", result, before.elapsed());
    }

    fn challenge2(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge4::challenge2(&input);


        println!("4-2: Result: {} (took {:.1?})", result, before.elapsed());
    }
}

mod day5 {
    use aoc2021::{challenge5, input};
    use std::time::Instant;
    use aoc2021::challenge5::parse_coordinates;

    pub fn run() {
        let input = input::read("input/challenge5.txt").unwrap();

        println!("Day 5:");
        print!("\t");
        challenge1(&input);
        print!("\t");
        challenge2(&input);
    }

    fn challenge1(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge5::challenge1(parse_coordinates(&input));

        println!("5-1: Result: {} (took {:.1?})", result, before.elapsed());
    }

    fn challenge2(input: &Vec<String>) {
        let before = Instant::now();
        let result = challenge5::challenge2(parse_coordinates(&input));

        println!("5-2: Result: {} (took {:.1?})", result, before.elapsed());
    }
}

mod day6 {
    use std::fs;
    use aoc2021::{challenge6};
    use std::time::Instant;

    pub fn run() {
        let input = fs::read_to_string("input/challenge6.txt").unwrap().trim().to_string();

        println!("Day 6:");
        print!("\t");
        challenge1(&input);
        print!("\t");
        challenge2(&input);
    }

    fn challenge1(input: &String) {
        let before = Instant::now();
        let result = challenge6::challenge2(&input, 80);

        println!("6-1: Result: {} (took {:.1?})", result, before.elapsed());
    }

    fn challenge2(input: &String) {
        let before = Instant::now();
        let result = challenge6::challenge2(&input, 256);

        println!("6-2: Result: {} (took {:.1?})", result, before.elapsed());
    }
}

mod day7 {
    use std::fs;
    use aoc2021::{input, challenge7};
    use std::time::Instant;

    pub fn run() {
        let input = input::read_numbers("input/challenge7.txt").unwrap();

        println!("Day 7:");
        print!("\t");
        challenge1(&input);
        print!("\t");
        challenge2(&input);
    }

    fn challenge1(input: &[i32]) {
        let before = Instant::now();
        let result = challenge7::challenge1(&input);

        println!("7-1: Result: {} (took {:.1?})", result, before.elapsed());
    }

    fn challenge2(input: &[i32]) {
        let before = Instant::now();
        let result = challenge7::challenge2(&input);

        println!("7-2: Result: {} (took {:.1?})", result, before.elapsed());
    }
}