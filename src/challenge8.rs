use std::collections::{HashSet};

#[derive(Eq, PartialEq, Debug)]
struct InputLine {
    input_values: Vec<String>,
    output_values: Vec<String>,
}

#[derive(Eq, PartialEq, Debug)]
struct OutputLine {
    input_digits: Vec<u8>,
    output_digits: Vec<u8>,
}

fn parse_line(input: &String) -> InputLine {
    let mut parts = input.split(" | ");
    let input_values = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|val| val.to_string())
        .collect::<Vec<_>>();
    let output_values = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|val| val.to_string())
        .collect::<Vec<_>>();

    InputLine {
        input_values,
        output_values,
    }
}

pub fn challenge1(input: &[String]) -> usize {
    let out_segments = input
        .iter()
        .map(|val| parse_line(val).output_values)
        .collect::<Vec<_>>();
    out_segments
        .iter()
        .map(|val| {
            val.iter()
                .filter(|seg| match seg.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

pub fn challenge2(input: &[String]) -> u32 {
    let input = input.iter().map(|val| parse_line(val)).collect::<Vec<_>>();

    input.iter().map(&decode_line).sum()
}

fn decode_line(line: &InputLine) -> u32 {
    let input = line.input_values.iter().map(|item| item.as_bytes().iter().collect::<HashSet<_>>()).collect::<Vec<_>>();

    let one = input.iter().find(|val| val.len() == 2).unwrap();
    let four = input.iter().find(|val| val.len() == 4).unwrap();
    let seven = input.iter().find(|val| val.len() == 3).unwrap();
    let eight = input.iter().find(|val| val.len() == 7).unwrap();

    let fives = input.iter().filter(|val| val.len() == 5).collect::<Vec<_>>();
    let sixes = input.iter().filter(|val| val.len() == 6).collect::<Vec<_>>();


    // six is the only "sixes"-digit that contains no segments of one.
    let six = *sixes.iter().find(|seq| !seq.is_superset(one)).unwrap();
    // nine is the only "sixes"-digit that contains all elements of four.
    let nine = *sixes.iter().find(|seq| seq.is_superset(four)).unwrap();
    // Zero is the remaining "sixes"-digit
    let zero = *sixes.iter().find(|seq| **seq != nine && **seq != six).unwrap();

    // three is the only "fives"-digit that contains both segments of one.
    let three = *fives.iter().find(|seq| seq.is_superset(one)).unwrap();
    let five = *fives.iter().find(|seq| seq.is_subset(six)).unwrap();
    let two = *fives.iter().find(|seq| **seq != three && **seq != five).unwrap();

    let lookup = vec![zero, one, two, three, four, five, six, seven, eight, nine];

    // Decode output
    let out = line.output_values.iter().map(|val| {
        let val = val.as_bytes().iter().collect::<HashSet<_>>();
        let (val, _) = lookup.iter().enumerate().find(|(_, comp)| val == ***comp).unwrap();
        val as u8
    }).collect::<Vec<_>>();
    let str = format!("{}{}{}{}", out[0], out[1], out[2], out[3]);
    str.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        let input =
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
                .to_string();
        let result = super::parse_line(&input);

        let expected = InputLine {
            input_values: vec![
                "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac",
                "fegbdc",
            ]
            .iter()
            .map(|val| val.to_string())
            .collect(),
            output_values: vec!["fgae", "cfgab", "fg", "bagce"]
                .iter()
                .map(|val| val.to_string())
                .collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn aoc_example1() {
        let input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let input = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();
        let result = challenge1(&input);
        assert_eq!(result, 26);
    }

    #[test]
    fn decode_single() {
        let input = super::parse_line(&"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string());
        let result = decode_line(&input);
        assert_eq!(result, 5353);
    }

    #[test]
    fn aoc_example2() {
        let input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let input = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();
        let result = challenge2(&input);
        assert_eq!(result, 61229);
    }
}
