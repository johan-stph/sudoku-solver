use crate::Field::{EMPTY, GUESS, VALID};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Field {
    VALID(i8),
    GUESS(i8),
    EMPTY,
}

impl Default for Field {
    fn default() -> Self {
        EMPTY
    }
}


#[derive(Debug)]
struct Board {
    board: Vec<Field>,
    placed: usize,
}

impl Board {
    fn create_board(board_vec: Vec<Vec<i8>>) -> Self {
        let board_thing = board_vec.iter().flatten().map(
            |x| {
                if *x == 0 {
                    return EMPTY;
                }
                VALID(*x)
            }
        );
        let board = board_thing.collect::<Vec<Field>>();
        let placed = board.iter().filter(|&&x| x != EMPTY).count();
        Board {
            board,
            placed,
        }
    }
}


impl Board {
    fn solve_board(&mut self) -> bool {
        if self.placed == 81 {
            return true;
        }

        // Find the first empty cell
        let mut empty_index = None;
        for (i, &field) in self.board.iter().enumerate() {
            if field == EMPTY {
                empty_index = Some(i);
                break;
            }
        }

        //the cell has to exist because of the guard condition self.placed = 81
        let empty_index = empty_index.unwrap();
        for i in 1..=9 {
            self.board[empty_index] = GUESS(i);
            self.placed += 1;
            if self.check_valid(empty_index) {
                if self.solve_board() {
                    return true;
                }
            }
            self.board[empty_index] = EMPTY;
            self.placed -= 1;
        }

        false
    }


    fn check_valid(&self, index: usize) -> bool {
        let row = index / 9;
        let col = index % 9;

        // Check row
        for i in 0..9 {
            let check_index = row * 9 + i;
            if check_index == index {
                continue;
            }
            if let GUESS(x) | VALID(x) = self.board[check_index] {
                if let GUESS(y) = self.board[index] {
                    if x == y {
                        return false;
                    }
                }
            }
        }

        // Check column
        for i in 0..9 {
            let check_index = i * 9 + col;
            if check_index == index {
                continue;
            }
            if let GUESS(x) | VALID(x) = self.board[check_index] {
                if let GUESS(y) = self.board[index] {
                    if x == y {
                        return false;
                    }
                }
            }
        }

        // Check 3x3 square
        let start_row = row / 3 * 3;
        let start_col = col / 3 * 3;
        for i in 0..3 {
            for j in 0..3 {
                let check_index = (start_row + i) * 9 + (start_col + j);
                if check_index == index {
                    continue;
                }
                if let GUESS(x) | VALID(x) = self.board[check_index] {
                    if let GUESS(y) = self.board[index] {
                        if x == y {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::Board;

    #[test]
    fn test_board_init() {
        let board_pos = vec![
            vec![0_i8, 7, 0, 0, 0, 4, 1, 3, 0],
            vec![0, 0, 0, 2, 0, 7, 0, 0, 6],
            vec![0, 0, 5, 0, 1, 3, 0, 2, 0],
            vec![0, 0, 1, 0, 0, 2, 0, 0, 0],
            vec![0, 0, 2, 1, 9, 0, 0, 5, 7],
            vec![0, 0, 3, 0, 4, 5, 8, 0, 2],
            vec![0, 1, 0, 3, 7, 8, 2, 6, 0],
            vec![3, 6, 7, 0, 0, 0, 5, 8, 0],
            vec![8, 0, 9, 0, 0, 1, 0, 7, 0],
        ];

        let mut board = Board::create_board(board_pos);
        //dbg!(&board);
        //board.board[0] = Field::GUESS(8);
        //println!("{}", board.check_valid(0));
        let res = board.solve_board();
        assert!(res)
    }
    #[test]
    fn test_board_v2() {
        let board_pos = vec![
            vec![9_i8, 0, 3, 0, 8, 0, 0, 0, 0],
            vec![0, 6, 0, 1, 0, 0, 7, 3, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 4],
            vec![0, 0, 0, 4, 0, 0, 0, 0, 6],
            vec![0, 0, 8, 0, 1, 0, 5, 4, 0],
            vec![0, 5, 0, 0, 0, 0, 0, 0, 2],
            vec![0, 7, 0, 3, 0, 0, 1, 5, 0],
            vec![0, 0, 6, 0, 0, 7, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 2, 0, 0]
        ];

        let mut board = Board::create_board(board_pos);
        //dbg!(&board);
        //board.board[0] = Field::GUESS(8);
        //println!("{}", board.check_valid(0));
        let res = board.solve_board();
        dbg!(board);
        assert!(res)
    }
}