#[derive(Eq, PartialEq, Debug)]
struct Grid {
    values: Vec<u8>,
    flashed: Vec<bool>,
    size: usize,
}

impl Grid {
    pub fn get_at(&self, x: usize, y: usize) -> u8 {
        self.values[self.get_index(x, y)]
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    pub fn unflashed_neighbors_at(&self, i: usize) -> Vec<usize> {
        let x = i % self.size;
        let y = i / self.size;

        let y_min = if y > 0 { y - 1 } else { y };
        let y_max = if y + 1 < self.size { y + 1 } else { y };
        let x_min = if x > 0 { x - 1 } else { x };
        let x_max = if x + 1 < self.size { x + 1 } else { x };

        let mut res = Vec::new();
        for n_y in y_min..=y_max {
            for n_x in x_min..=x_max {
                if !(n_y == y && n_x == x) && !self.flashed[self.get_index(n_x, n_y)] {
                    res.push(self.get_index(n_x, n_y));
                }
            }
        }
        res
    }

    pub fn unflashed_values(&self) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .zip(&self.flashed)
            .filter_map(
                |((i, val), flashed)| {
                    if !*flashed && *val > 9 {
                        Some(i)
                    } else {
                        None
                    }
                },
            )
            .collect::<Vec<_>>()
    }

    pub fn reset_flashed(&mut self) {
        self.flashed = vec![false; self.values.len()];
    }
}

fn parse(input: &[String]) -> Grid {
    let size = input.len();
    let values = input.iter().fold(vec![], |mut acc, line| {
        acc.append(
            &mut line
                .as_bytes()
                .iter()
                .map(|byte| byte - b'0')
                .collect::<Vec<_>>(),
        );
        acc
    });
    Grid {
        size,
        flashed: vec![false; values.len()],
        values,
    }
}

fn grid_format(grid: &Grid) -> String {
    let mut out = String::new();
    for i in 0..grid.size * grid.size {
        if i % grid.size == 0 {
            out.push('\n')
        }
        out.push((grid.values[i] + b'0') as char);
    }
    out
}

pub fn challenge1(input: &[String], steps: u32) -> u32 {
    let mut grid = parse(&input);
    println!("Before:{}", grid_format(&grid));

    let mut flashes = 0;
    for step in 0..steps {
        // Increase all values by one
        grid.values.iter_mut().for_each(|val| *val += 1);

        // Failure #1
        // let mut to_flash = grid.values.iter().enumerate().filter(|(_, &val)| val > 9).map(|(i, _)| i).collect::<Vec<_>>();
        // let mut flashed = Vec::new();
        // while to_flash.len() > 0 {
        //     for i in to_flash.clone() {
        //         grid.neighbors_at(i).iter_mut().for_each(|n_i| {
        //             if !flashed.contains(n_i) && grid.values[*n_i] >= 9 {
        //                 to_flash.push(*n_i);
        //             }
        //             grid.values[*n_i] += 1;
        //             flashed.push(i);
        //         });
        //     }
        // }

        // Failure #2
        let mut unflashed_in_grid = grid.unflashed_values();
        while unflashed_in_grid.len() > 0 {
            for i in unflashed_in_grid {
                let unflashed_neighbors = grid.unflashed_neighbors_at(i);
                for val in unflashed_neighbors{
                    grid.values[val] += 1
                }
                grid.flashed[i] = true;
            }
            unflashed_in_grid = grid.unflashed_values();
        }

        let mut count = 0u32;
        for i in 0..grid.size * grid.size {
            if grid.flashed[i] {
                grid.values[i] = 0;
                grid.flashed[i] = false;
                count += 1;
            }
        }

        // println!("Before:{}", grid_format(&grid));

        flashes += count;
    }
    flashes
}

pub fn challenge2(input: &[String]) -> u32 {
    let mut grid = parse(&input);
    println!("Before:{}", grid_format(&grid));

    let mut last_count = 0;
    let mut step = 0;
    while last_count < (grid.size * grid.size) as u32 {
        // Increase all values by one
        grid.values.iter_mut().for_each(|val| *val += 1);

        // Failure #2
        let mut unflashed_in_grid = grid.unflashed_values();
        while unflashed_in_grid.len() > 0 {
            for i in unflashed_in_grid {
                let unflashed_neighbors = grid.unflashed_neighbors_at(i);
                for val in unflashed_neighbors{
                    grid.values[val] += 1
                }
                grid.flashed[i] = true;
            }
            unflashed_in_grid = grid.unflashed_values();
        }

        let mut count = 0u32;
        for i in 0..grid.size * grid.size {
            if grid.flashed[i] {
                grid.values[i] = 0;
                grid.flashed[i] = false;
                count += 1;
            }
        }
        last_count = count;
        step += 1;
    }
    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grid() {
        let input = "1234\n2345\n3456\n4567"
            .lines()
            .map(|val| val.to_string())
            .collect::<Vec<_>>();
        let result = parse(&input);
        let expected = Grid {
            values: vec![1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7],
            flashed: vec![false; 16],
            size: 4,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn aoc_example1() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            .lines()
            .map(|val| val.to_string())
            .collect::<Vec<_>>();
        let result = challenge1(&input, 100);
        assert_eq!(result, 1656);
    }
}
