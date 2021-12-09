use std::collections::VecDeque;

struct Heightmap {
    values: Vec<u8>,
    width: usize,
    height: usize,
}

impl Heightmap {
    fn get_at(&self, x: usize, y: usize) -> u8 {
        self.values[x + y * self.width]
    }
}

fn get_low_points(heightmap: &Heightmap) -> Vec<(usize, usize)> {
    let mut lows = Vec::new();
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            let current = heightmap.get_at(x, y);

            if x > 0 {
                let left = heightmap.get_at(x - 1, y);
                if left < current {
                    continue;
                }
            }
            if x < heightmap.width - 1 {
                let right = heightmap.get_at(x + 1, y);
                if right < current {
                    continue;
                }
            }
            if y > 0 {
                let up = heightmap.get_at(x, y - 1);
                if up < current {
                    continue;
                }
            }
            if y < heightmap.height - 1 {
                let down = heightmap.get_at(x, y + 1);
                if down < current {
                    continue;
                }
            }

            if current == 9 {
                continue;
            }

            lows.push((x, y));
        }
    }
    lows
}

pub fn challenge1(input: &[String]) -> u32 {
    let heightmap = parse_heightmap(input);
    let lows = get_low_points(&heightmap);
    lows.iter()
        .map(|(x, y)| heightmap.get_at(*x, *y) as u32 + 1)
        .sum()
}

pub fn challenge2(input: &[String]) -> u32 {
    let heightmap = parse_heightmap(input);
    let lows = get_low_points(&heightmap);
    let mut basins = Vec::new();

    for low in lows {
        let mut basin_size = 0u32;

        let mut open_list = VecDeque::new();
        let mut visited = Vec::new();
        open_list.push_back(low);

        while open_list.len() > 0 {
            let (x, y) = open_list.pop_front().unwrap();
            visited.push((x, y));

            if x > 0 {
                let coords = (x - 1, y);
                let left = heightmap.get_at(coords.0, coords.1);
                if left < 9 {
                    if !open_list.contains(&coords) && !visited.contains(&coords) {
                        open_list.push_back(coords)
                    }
                }
            }
            if x < heightmap.width - 1 {
                let coords = (x + 1, y);
                let right = heightmap.get_at(coords.0, coords.1);
                if right < 9 {
                    if !open_list.contains(&coords) && !visited.contains(&coords) {
                        open_list.push_back(coords)
                    }
                }
            }
            if y > 0 {
                let coords = (x, y - 1);
                let up = heightmap.get_at(coords.0, coords.1);
                if up < 9 {
                    if !open_list.contains(&coords) && !visited.contains(&coords) {
                        open_list.push_back(coords)
                    }
                }
            }
            if y < heightmap.height - 1 {
                let coords = (x, y + 1);
                let down = heightmap.get_at(coords.0, coords.1);
                if down < 9 {
                    if !open_list.contains(&coords) && !visited.contains(&coords) {
                        open_list.push_back(coords)
                    }
                }
            }

            basin_size += 1;
        }

        basins.push(basin_size);
    }

    basins.sort();
    basins.iter().rev().take(3).product()
}

fn parse_heightmap(input: &[String]) -> Heightmap {
    let mut parsed = Vec::new();
    for line in input {
        parsed.append(
            &mut line
                .as_bytes()
                .iter()
                .map(|val| *val - 48)
                .collect::<Vec<_>>(),
        );
    }
    Heightmap {
        values: parsed,
        width: input[0].len(),
        height: input.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example1() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let input = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();
        let result = challenge1(&input);
        assert_eq!(result, 15);
    }

    #[test]
    fn nines() {
        let input = "\
999999999
999999999
999999999
999999999";
        let input = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();
        let result = challenge1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn aoc_example2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let input = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();
        let result = challenge2(&input);
        assert_eq!(result, 1134);
    }
}
