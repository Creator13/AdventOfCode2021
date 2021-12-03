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
        let result = get_gamma_epsilon_binary(&input_vec);

        assert_eq!(
            result,
            Rates {
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
        let result = get_gamma_epsilon_binary(&input_vec);

        assert_eq!(
            result,
            Rates {
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
        let result = get_gamma_epsilon_logical(&input_vec);

        assert_eq!(
            result,
            Rates {
                gamma: 22,
                epsilon: 9
            }
        );
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Rates {
    gamma: u32,
    epsilon: u32,
}

impl Rates {
    pub fn power_consumption(&self) -> u32 {
        self.gamma * self.epsilon
    }
}

pub fn get_gamma_epsilon_binary(input: &[String]) -> Rates {
    // Convert the binary strings to integers.
    let lines: Vec<u32> = input
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    let mut counter: Vec<u32> = std::iter::repeat(0).take(input[0].len()).collect();

    for line in lines {
        let mut mask: u32 = 1;
        let mut i: usize = 0;
        // Loop over each bit in the number (mask <== 1). If the bit is set (line & mask > 0), add 1.
        // The resulting vector will have the least-significant bit on the right instead of on the left, unlike the input integers which have the LSB on the left.
        // This makes it easier to convert the counter to a number, because the vector iterator will start at the least-significant bit.
        while mask <= line {
            let res = line & mask > 0;
            counter[i] += res as u32;
            mask <<= 1;
            i += 1;
        }
    }

    let gamma = u32_from_counter_slice(&counter, input.len());

    // mask off as many bits as there are significant bits in the input strings.
    let flip_mask: u32 = (1 << input[0].len()) - 1;
    Rates {
        gamma,
        epsilon: (!gamma) & flip_mask,
    }
}

pub fn get_gamma_epsilon_logical(input: &[String]) -> Rates {
    let lines: Vec<&[u8]> = input.iter().map(|line| line.as_bytes()).collect();

    let mut counter: Vec<u32> = std::iter::repeat(0).take(input[0].len()).collect();

    for line in lines {
        for (i, x) in line.iter().enumerate() {
            counter[i] += (*x as char == '1') as u32;
        }
    }

    counter.reverse();

    let gamma= u32_from_counter_slice(&counter, input.len());

    // mask off as many bits as there are significant bits in the input strings.
    let flip_mask: u32 = (1 << input[0].len()) - 1;
    Rates {
        gamma,
        epsilon: (!gamma) & flip_mask,
    }
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