use std::collections::HashMap;

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
        let result = challenge2(&input, 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn aoc_example2() {
        let input = String::from("3,4,3,1,2");
        let result = challenge2(&input, 256);
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

pub fn challenge2(input: &String, days: usize) -> u64 {
    let input: Vec<_> = input.split(",").map(|val| val.parse().unwrap()).collect();
    let mut fisheses :HashMap<u8, _> = HashMap::from([0, 1, 2, 3, 4, 5, 6, 7, 8].map(|val| (val, 0)));

    // Add initial fishes
    for x in input {
        fisheses.entry(x).and_modify(|val| *val += 1);
    }

    let fish_len = fisheses.len() as u8;

    for _ in 0..days {
        let to_add = *fisheses.entry(0).or_default();
        for i in 0..fish_len - 1 {
            *(fisheses.get_mut(&i).unwrap()) = *fisheses.get(&(i + 1)).unwrap();
        }
        *(fisheses.get_mut(&6).unwrap()) += to_add;
        *(fisheses.get_mut(&8).unwrap()) = to_add;
    }

    fisheses.iter().fold(0, |acc, (_, val)| acc + val)
}