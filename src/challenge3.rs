#[cfg(test)]
mod tests {
    use super::*;
    use std::string::String;

    #[test]
    pub fn aoc_example1_binary() {
        let input = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = get_power_rates_binary(&input_vec);

        println!("PowerRates: {:?}", result);

        assert_eq!(
            result,
            PowerRates {
                gamma: 22,
                epsilon: 9
            }
        );
    }

    #[test]
    pub fn first_bit_true() {
        let input = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
11111
00001
00001";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = get_power_rates_binary(&input_vec);

        assert_eq!(
            result,
            PowerRates {
                gamma: 23,
                epsilon: 8
            }
        );
    }

    #[test]
    pub fn aoc_example1_logical() {
        let input = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = get_power_rates_logical(&input_vec);

        assert_eq!(
            result,
            PowerRates {
                gamma: 22,
                epsilon: 9
            }
        );
    }

    #[test]
    pub fn aoc_example2() {
        let input = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = get_life_rates(&input_vec);

        assert_eq!(
            result,
            LifeRates {
                oxygen: 23,
                co2: 10,
            }
        );
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct PowerRates {
    gamma: u32,
    epsilon: u32,
}

impl PowerRates {
    pub fn power_consumption(&self) -> u32 {
        self.gamma * self.epsilon
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct LifeRates {
    oxygen: u32,
    co2: u32,
}

impl LifeRates {
    pub fn life_rating(&self) -> u32 {
        self.oxygen * self.co2
    }
}

pub fn get_power_rates_binary(input: &[String]) -> PowerRates {
    let bit_count = input[0].len();

    // Convert the binary strings to integers.
    let lines: Vec<u32> = input
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    let counter = count_ones_binary(&lines, bit_count);

    let gamma = u32_from_counter_slice(&counter, input.len());

    // mask off as many bits as there are significant bits in the input strings.
    let flip_mask: u32 = (1 << input[0].len()) - 1;
    PowerRates {
        gamma,
        epsilon: (!gamma) & flip_mask,
    }
}

pub fn get_power_rates_logical(input: &[String]) -> PowerRates {
    let lines: Vec<&[u8]> = input.iter().map(|line| line.as_bytes()).collect();

    let counter = count_ones_logical(&lines, input[0].len());

    let gamma = u32_from_counter_slice(&counter, input.len());

    // mask off as many bits as there are significant bits in the input strings.
    let flip_mask: u32 = (1 << input[0].len()) - 1;
    PowerRates {
        gamma,
        epsilon: (!gamma) & flip_mask,
    }
}

pub fn get_life_rates(input: &[String]) -> LifeRates {
    let input = input.iter().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let mut iteration_values = input.clone();
    for i in 0..input[0].len() {
        let half_length_floor = iteration_values.len() as u32 >> 1;
        let half_length_ceil = iteration_values.len() as u32 - half_length_floor;
        let count_ones = iteration_values
            .iter()
            .fold(0, |acc: u32, val| acc + (val[i] == b'1') as u32);
        let most_common = if count_ones >= half_length_ceil
        {
            '1'
        } else {
            '0'
        } as u8;

        let mut result = Vec::new();
        for line in iteration_values {
            if line[i] == most_common {
                result.push(line);
            }
        }

        // println!(
        //     "{}/{} ones, Most common: {}, {:?}",
        //     count_ones,
        //     half_length_floor + half_length_ceil,
        //     most_common as char,
        //     result
        // );

        iteration_values = result;

        if iteration_values.len() == 1 {
            break;
        }
    }

    let oxygen = iteration_values[0];

    let mut iteration_values = input.clone();
    for i in 0..input[0].len() {
        let half_length_floor = iteration_values.len() as u32 >> 1;
        let half_length_ceil = iteration_values.len() as u32 - half_length_floor;
        let count_ones = iteration_values
            .iter()
            .fold(0, |acc: u32, val| acc + (val[i] == b'1') as u32);
        let least_common = if count_ones < half_length_ceil
        {
            '1'
        } else {
            '0'
        } as u8;

        let mut result = Vec::new();
        for line in iteration_values {
            if line[i] == least_common {
                result.push(line);
            }
        }

        // println!(
        //     "{}/{} ones, Least common: {}, {:?}",
        //     count_ones,
        //     half_length_floor + half_length_ceil,
        //     least_common as char,
        //     result
        // );

        iteration_values = result;

        if iteration_values.len() == 1 {
            break;
        }
    }

    let co2 = iteration_values[0];

    let ox_string = std::str::from_utf8(oxygen).unwrap();
    let co_string = std::str::from_utf8(co2).unwrap();

    let oxygen_int = u32::from_str_radix(ox_string, 2).unwrap();
    let co2_int = u32::from_str_radix(co_string, 2).unwrap();

    // println!("ox: {:?} -> {} -> {} | co: {:?} -> {} -> {}", oxygen, ox_string, oxygen_int, co2, co_string, co2_int);

    LifeRates { co2: co2_int, oxygen: oxygen_int }
}

fn count_ones_binary(input: &[u32], bit_count: usize) -> Vec<u32> {
    let mut counter: Vec<u32> = std::iter::repeat(0).take(bit_count).collect();

    for line in input {
        let mut mask: u32 = 1;
        let mut i: usize = 0;
        // Loop over each bit in the number (mask <== 1). If the bit is set (line & mask > 0), add 1.
        // The resulting vector will have the least-significant bit on the right instead of on the left, unlike the input integers which have the LSB on the left.
        // This makes it easier to convert the counter to a number, because the vector iterator will start at the least-significant bit.
        while mask <= *line {
            let res = *line & mask > 0;
            counter[i] += res as u32;
            mask <<= 1;
            i += 1;
        }
    }

    counter
}

fn count_ones_logical(input: &[&[u8]], bit_count: usize) -> Vec<u32> {
    let mut counter: Vec<u32> = std::iter::repeat(0).take(bit_count).collect();

    for line in input {
        for (i, x) in line.iter().enumerate() {
            counter[i] += (*x as char == '1') as u32;
        }
    }

    counter.reverse();
    counter
}

fn u32_from_counter_slice(slice: &[u32], input_length: usize) -> u32 {
    let half_length = input_length as u32 >> 1;
    slice.iter().enumerate().fold(0, |acc, (i, x)| {
        // "If the number at position i (x) is larger than than the half_length" means that there are more 1s than 0s at position i in the input list
        // If this is the case, set bit i (from left) to 1
        // Comparison is >=, due to integer division flooring the result)
        acc + if *x > half_length { 1 << i } else { 0 }
    })
}
