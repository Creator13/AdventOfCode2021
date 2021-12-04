#[cfg(test)]
mod tests {
    use super::*;

    fn get_board() -> Board {
        let input = "\
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        Board::from(&input_vec)
    }

    #[test]
    fn make_board() {
        let input = "\
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        // Construct a valid board (todo!("Should move this to a separate function"))
        let board_nums: [[u8; 5]; 5] = [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ];
        let size = board_nums[0].len();
        let values = board_nums.concat();
        let markers = std::iter::repeat(false).take(values.len()).collect();
        let board = Board {
            size,
            values,
            markers,
        };

        assert_eq!(Board::from(&input_vec), board);
    }

    #[test]
    fn parse() {
        let input = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        let (draws, _) = parse_input(&input_vec);

        assert_eq!(
            draws,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        )
    }

    #[test]
    #[rustfmt::skip]
    fn win_direct_by_row() {
        let mut board = get_board();
        board.markers = vec![
            true, true, true, true, true,
            false, false, false, false, false,
            false, false, true, false, false,
            true, false, true, false, true,
            false, false, false, false, false,
        ];
        assert_eq!(board.wins(), true);
    }

    #[test]
    #[rustfmt::skip]
    fn win_direct_by_col() {
        let mut board = get_board();
        board.markers = vec![
            false, false, false, true, false,
            false, true, false, true, false,
            true, false, false, true, false,
            false, false, false, true, false,
            false, true, false, true, true
        ];
        assert_eq!(board.wins(), true);
    }

    #[test]
    #[rustfmt::skip]
    fn win_direct_by_col_summary() {
        let mut board = get_board();
        board.markers = vec![
            false, false, false, true, false,
            false, true, false, true, false,
            true, false, false, true, false,
            false, false, false, true, false,
            false, true, false, true, true
        ];
        assert_eq!(board.summary(), 22 + 13 + 17 + 0 + 8 + 23 + 24 + 9 + 14 + 7 + 6 + 10 + 3 + 5 + 1 + 20);
    }

    #[test]
    #[rustfmt::skip]
    fn lose_direct() {
        let mut board = get_board();
        board.markers = vec![
            false, false, false, true, false,
            true, true, true, true, false,
            true, false, false, true, false,
            false, false, true, false, false,
            false, true, false, true, true
        ];
        assert_eq!(board.wins(), false);
    }

    #[test]
    fn win_by_sequence() {
        let mut board = get_board();
        let seq: Vec<u8> = vec![13, 23, 16, 17, 20, 21, 14, 0, 50, 3, 1];

        let mut i: usize = 0;
        for val in seq {
            board.mark(val);
            if board.wins() {
                break;
            }
            i += 1;
        }

        assert_eq!(board.wins(), true);
        assert_eq!(i, 9, "Sequence iteration quit when board had not won");
    }

    #[test]
    fn lose_by_sequence() {
        let mut board = get_board();
        let seq: Vec<u8> = vec![13, 23, 16, 17, 20, 21, 14, 0, 50, 9, 1];

        let mut i: usize = 0;
        for val in seq.iter() {
            board.mark(*val);
            if board.wins() {
                break;
            }
            i += 1;
        }

        assert_eq!(board.wins(), false);
        assert_eq!(
            i,
            seq.len(),
            "Sequence iteration quit when board had not lost"
        );
    }

    #[test]
    fn aoc_example1() {
        let input = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let (draws, mut board) = parse_input(&input_vec);

        let (winner, winning_draw) = first_winning_board(&draws, &mut board);

        assert_eq!(winning_draw, 24);
        assert_eq!(winner.summary(), 188);
    }

    #[test]
    fn aoc_example2() {
        let input = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let input_vec: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let (draws, mut board) = parse_input(&input_vec);

        let (winner, winning_draw) = last_winning_board(&draws, &mut board);

        assert_eq!((winning_draw, winner.summary()), (13, 148));
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Board {
    size: usize,
    values: Vec<u8>,
    markers: Vec<bool>,
}

impl Board {
    pub fn mark(&mut self, val: u8) {
        let val_i = self.values.iter().position(|board_val| *board_val == val);
        match val_i {
            Some(val) => self.markers[val] = true,
            None => return,
        }
    }

    pub fn wins(&self) -> bool {
        let has_winning_row = self
            .markers
            .chunks(self.size)
            .any(|chunk| chunk.iter().all(|val| *val == true));

        if has_winning_row {
            return true;
        }

        let mut has_winning_col = false;
        for x in 0..self.size {
            let mut acc = self.markers[x];
            for y in 0..self.size {
                acc &= self.markers[y * self.size + x];
            }
            has_winning_col |= acc;
        }

        has_winning_col
    }

    pub fn summary(&self) -> u32 {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, val)| match self.markers[i] {
                true => None,
                false => Some(*val as u32),
            })
            .sum()
    }

    pub fn from(input: &[String]) -> Self {
        let lines = input
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let size = lines[0].len();

        let values = lines.concat();
        let markers = std::iter::repeat(false).take(values.len()).collect();

        Self {
            size,
            values,
            markers,
        }
    }
}

pub fn parse_input(lines: &[String]) -> (Vec<u8>, Vec<Board>) {
    // split by empty lines
    let mut lines = lines.split(|el| el.is_empty());

    // Parse draws
    let draws = lines
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(|char| char == ',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    let boards = lines
        .map(|board_slice| Board::from(board_slice))
        .collect::<Vec<_>>();

    return (draws, boards);
}

pub fn first_winning_board<'a>(draws: &[u8], boards: &'a mut Vec<Board>) -> (&'a Board, u8) {
    let mut winning_board = 0;
    let mut winning_draw = 0;
    'draw_loop: for (i, draw) in draws.iter().enumerate() {
        for (j, board) in boards.iter_mut().enumerate() {
            board.mark(*draw);
            if board.wins() {
                winning_board = j;
                winning_draw = i;
                break 'draw_loop;
            }
        }
    }

    (&boards[winning_board], draws[winning_draw])
}

fn last_winning_board<'a>(draws: &[u8], boards: &'a mut Vec<Board>) -> (&'a Board, u8) {
    let mut winning_board = 0;
    let mut winning_draw = 0;
    for (i, draw) in draws.iter().enumerate() {
        for (j, board) in boards.iter_mut().enumerate() {
            if board.wins() {continue};

            board.mark(*draw);
            if board.wins() {
                winning_board = j;
                winning_draw = i;
            }
        }
    }

    (&boards[winning_board], draws[winning_draw])
}

pub fn challenge1(input: &[String]) -> u32 {
    let (draws, mut boards) = parse_input(input);
    let (b_win, d_win) = first_winning_board(&draws, &mut boards);

    b_win.summary() * d_win as u32
}

pub fn challenge2(input: &[String]) -> u32 {
    let (draws, mut boards) = parse_input(input);
    let (b_win, d_win) = last_winning_board(&draws, &mut boards);

    b_win.summary() * d_win as u32
}
