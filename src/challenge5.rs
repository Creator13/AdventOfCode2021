use std::cmp::{max, min};

#[cfg(test)]
mod tests {
    use crate::input::read_lines;
    use super::*;

    #[test]
    fn parse() {
        let input = [String::from("0,9 -> 5,9"), String::from("8,0 -> 0,8")];
        let result = parse_coordinates(&input);
        let expected = VentMap {
            vents: vec![((0, 9), (5, 9)), ((8, 0), (0, 8))],
            height: 9,
            width: 8,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn aoc_example1() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let input_vec = input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        let result = challenge1(parse_coordinates(&input_vec));

        assert_eq!(result, 5);
    }

    #[test]
    fn aoc_example2() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let input_vec = input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        let result = challenge2(parse_coordinates(&input_vec));

        assert_eq!(result, 12);
    }

    #[test]
    fn parse_read() {
        let input = read_lines("input/challenge5.txt").unwrap();
        let result = parse_coordinates(&input);
        assert_eq!(result.vents.len(), 500);
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct VentMap {
    vents: Vec<((usize, usize), (usize, usize))>,
    width: usize,
    height: usize,
}

pub fn parse_coordinates(input: &[String]) -> VentMap {
    let mut max_x = 0;
    let mut max_y = 0;

    let coords = input
        .iter()
        .map(|line| {
            let mut coord_tuples = line.split("->").map(|mut coord| {
                coord = coord.trim();
                let mut iter = coord.split(",");
                let x = iter.next().unwrap().parse().unwrap();
                let y = iter.next().unwrap().parse().unwrap();
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
                (x, y)
            });
            (coord_tuples.next().unwrap(), coord_tuples.next().unwrap())
        })
        .collect::<Vec<_>>();

    VentMap {
        vents: coords,
        width: max_x,
        height: max_y,
    }
}

pub fn challenge1(input: VentMap) -> u32 {
    let width = input.width + 1;
    let mut visits = vec![0u8; width * width];

    for ((mut start_x, mut start_y), (mut end_x, mut end_y)) in input.vents {
        // Horizontal line
        if start_y == end_y {
            let y = start_y;

            if start_x > end_x {
                let tmp = start_x;
                start_x = end_x;
                end_x = tmp;
            }

            for i in (start_x + width * y)..=(end_x + width * y) {
                visits[i] += 1;
            }
        }
        // Vertical line
        else if start_x == end_x {
            let x = start_x;

            if start_y > end_y {
                let tmp = start_y;
                start_y = end_y;
                end_y = tmp;
            }

            for i in ((x + width * start_y)..=(x + width * end_y)).step_by(width) {
                visits[i] += 1;
            }
        }
    }

    visits.iter().filter(|val| **val >= 2).count() as u32
}

pub fn challenge2(input: VentMap) -> u32 {
    let width = input.width + 1;
    let mut visits = vec![0u8; width * width];

    for ((mut start_x, mut start_y), (mut end_x, mut end_y)) in input.vents {
        //Diagonal line
        let diff_x = start_x as i32 - end_x as i32;
        let diff_y= start_y as i32 - end_y as i32;
        if diff_x.abs() == diff_y.abs() {
            if diff_x == diff_y {
                let mut y = min(start_y, end_y);
                for x in min(start_x, end_x)..=max(start_x, end_x) {
                    visits[x + y * width] += 1;
                    y += 1
                }
            } else {
                let mut y = min(start_y, end_y);
                for x in (min(start_x, end_x)..=max(start_x, end_x)).rev() {
                    visits[x + y * width] += 1;
                    y += 1
                }
            }
        } else {
            // Horizontal line
            if start_y == end_y {
                let y = start_y;

                if start_x > end_x {
                    let tmp = start_x;
                    start_x = end_x;
                    end_x = tmp;
                }

                for i in (start_x + width * y)..=(end_x + width * y) {
                    visits[i] += 1;
                }
            }
            // Vertical line
            else if start_x == end_x {
                let x = start_x;

                if start_y > end_y {
                    let tmp = start_y;
                    start_y = end_y;
                    end_y = tmp;
                }

                for i in ((x + width * start_y)..=(x + width * end_y)).step_by(width) {
                    visits[i] += 1;
                }
            }
        }
    }

    visits.iter().filter(|val| **val >= 2).count() as u32
}
