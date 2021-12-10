use std::collections::HashMap;

fn openers() -> HashMap<u8, u8> {
    HashMap::from([
        (b'[', b']'),
        (b'(', b')'),
        (b'<', b'>'),
        (b'{', b'}'),
    ])
}

pub fn challenge1(input: &[String]) -> u32 {
    let openers = openers();
    let scores = HashMap::from([
        (b')', 3),
        (b']', 57),
        (b'}', 1197),
        (b'>', 25137),
    ]);

    let mut syntax_score = 0;

    'line_loop: for line in input {
        let mut stack = Vec::new();

        for c in line.as_bytes() {
            if openers.contains_key(c) {
                stack.push(c);
                continue;
            }

            let last_opener = *stack.last().unwrap();
            let expected_closer = openers.get(last_opener).unwrap();
            if *c == *expected_closer {
                stack.pop();
            }
            else {
                // println!("Line {} corrupted: expected {}, but found {} instead.", i, *expected_closer as char, *c as char);
                syntax_score += scores.get(c).unwrap();
                continue 'line_loop;
            }
        }

        if stack.len() == 0 {
            // println!("Line {} complete!", i)
        }
        else {
            // println!("Line {} incomplete!", i)
        }
    }
    syntax_score
}

pub fn challenge2(input: &[String]) -> u64 {
    let openers = openers();
    let scores = HashMap::from([
        (b')', 1),
        (b']', 2),
        (b'}', 3),
        (b'>', 4),

    ]);

    let mut line_scores = Vec::new();

    'line_loop: for line in input {
        let mut stack = Vec::new();

        for c in line.as_bytes() {
            if openers.contains_key(c) {
                stack.push(c);
                continue;
            }

            let last_opener = *stack.last().unwrap();
            let expected_closer = openers.get(last_opener).unwrap();
            if *c == *expected_closer {
                stack.pop();
            }
            else {
                // Line is corrupted -> skip
                continue 'line_loop;
            }
        }

        let mut completion_score = 0;

        // Line is incomplete: complete it
        while let Some(bracket) = stack.pop() {
            let close_with = openers.get(bracket).unwrap();
            completion_score *= 5;
            completion_score += scores.get(close_with).unwrap();
        }
        line_scores.push(completion_score);
    }

    line_scores.sort();
    line_scores[line_scores.len() / 2 ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_challenge1() {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".lines().map(|val| val.to_string()).collect::<Vec<_>>();
        let result = challenge1(&input);
        assert_eq!(result, 26397);
    }

    #[test]
    fn aoc_challenge2() {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".lines().map(|val| val.to_string()).collect::<Vec<_>>();
        let result = challenge2(&input);
        assert_eq!(result, 288957);
    }
}