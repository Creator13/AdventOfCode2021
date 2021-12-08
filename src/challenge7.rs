#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let input = input
            .split(",")
            .map(|val| val.parse().unwrap())
            .collect::<Vec<_>>();
        let result = challenge1(&input);
        assert_eq!(result, 37);
    }

    #[test]
    fn aoc_example2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let input = input
            .split(",")
            .map(|val| val.parse().unwrap())
            .collect::<Vec<_>>();
        let result = challenge2(&input);
        assert_eq!(result, 168);
    }
}

use std::cmp;

pub fn challenge1(input: &[i32]) -> i32 {
    let max_input = *input.iter().max().unwrap();
    let min_input = *input.iter().min().unwrap();

    let mut min_fuel = i32::MAX;

    for i in min_input..max_input {
        let fuel = input.iter().fold(0, |acc, val| acc + (*val - i).abs());
        min_fuel = cmp::min(min_fuel, fuel);
    }

    min_fuel
}

pub fn challenge2(input: &[i32]) -> i32 {
    let max_input = *input.iter().max().unwrap();
    let min_input = *input.iter().min().unwrap();

    let mut min_fuel = i32::MAX;

    for i in min_input..max_input {
        let fuel = input.iter().fold(0, |acc, val| {
            acc + (1..=(*val - i).abs()).sum::<i32>()
        });
        min_fuel = cmp::min(min_fuel, fuel);
    }

    min_fuel
}
