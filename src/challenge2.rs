#[cfg(test)]
mod tests {
    use std::string::String;
    use crate::challenge2::horizontal_pos_and_depth;

    #[test]
    fn aoc_example_c1() {
        let input= vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];

        assert_eq!(horizontal_pos_and_depth(&input), 150)
    }
}

pub fn horizontal_pos_and_depth(input: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;

    for x in input {
        let mut split = x.split(" ");

        let val = split.clone().last().unwrap();
        let val = val.parse::<i32>().unwrap();

        match split.nth(0).unwrap() {
            "down" => vertical += val,
            "up" => vertical -= val,
            "forward" => horizontal += val,
            _ => ()
        }
    }

    horizontal * vertical
}
