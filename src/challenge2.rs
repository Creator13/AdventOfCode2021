#[cfg(test)]
mod tests {
    use crate::challenge2::*;
    use std::string::String;

    #[test]
    fn aoc_example_c1() {
        let input = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];

        assert_eq!(move_by_aim(&input), 150)
    }

    #[test]
    fn aoc_example_c2() {
        let input = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];

        assert_eq!(move_by_aim(&input), 900)
    }
}

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn map_to_commands(input: &Vec<String>) -> Vec<Command> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            match split.next().unwrap() {
                "up" => Command::Up(split.next().unwrap().parse().unwrap()),
                "down" => Command::Down(split.next().unwrap().parse().unwrap()),
                "forward" => Command::Forward(split.next().unwrap().parse().unwrap()),
                _ => panic!("Unable to parse command"),
            }
        })
        .collect()
}

pub fn move_directly(input: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;

    let commands = map_to_commands(input);

    for x in commands {
        match x {
            Command::Up(val) => vertical -= val,
            Command::Down(val) => vertical += val,
            Command::Forward(val) => horizontal += val,
        }
    }

    horizontal * vertical
}

pub fn move_by_aim(input: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    let commands = map_to_commands(input);

    for x in commands {
        match x {
            Command::Up(val) => aim -= val,
            Command::Down(val) => aim += val,
            Command::Forward(val) => {
                horizontal += val;
                vertical += val * aim;
            }
        }
    }

    horizontal * vertical
}
