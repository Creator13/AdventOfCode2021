#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example1_1() {
        let input = String::from("3,4,3,1,2");
        let result = challenge1(&input, 18);
        assert_eq!(result, 26);
    }

    #[test]
    fn aoc_example1() {
        let input = String::from("3,4,3,1,2");
        let result = challenge1(&input, 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn aoc_example2() {
        let input = String::from("3,4,3,1,2");
        let result = challenge1(&input, 256);
        assert_eq!(result, 26984457539);
    }
}

pub fn challenge1(input: &String, days: usize) -> usize {
    let mut input: Vec<u8> = input.split(",").map(|val| val.parse().unwrap()).collect();
    for _ in 0..days {
        let to_add = input.iter().filter(|val| **val == 0).count();
        input
            .iter_mut()
            .for_each(|val| if *val > 0 { *val -= 1 } else { *val = 6 });
        input.append(&mut vec![8u8; to_add]);
    }
    input.len()
}
