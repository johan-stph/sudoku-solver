use serde::Serialize;
use thiserror::Error;

use crate::Field::{EMPTY, GUESS, VALID};
use crate::SudokuErrors::{InvalidBoardConfig, ParsingError, Unsolvable};

#[derive(Error, Debug)]
pub enum SudokuErrors {
    #[error("the provided Board config is not valid")]
    InvalidBoardConfig,
    #[error("there is an invalid character in the Board config")]
    ParsingError,
    #[error("the board has no valid solutions")]
    Unsolvable
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum Field {
    VALID(i8),
    GUESS(i8),
    EMPTY,
}

impl TryFrom<char> for Field {
    type Error = SudokuErrors;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c == '0' {
            return Ok(EMPTY);
        }
        if let Some(digit) = c.to_digit(10) {
            Ok(VALID(digit as i8))
        } else {
            Err(ParsingError)
        }
    }
}

impl Into<char> for Field {
    fn into(self) -> char {
        match self {
            EMPTY => '0',
            GUESS(x) | VALID(x) => char::from_digit(x as u32, 10).unwrap_or('#')
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        EMPTY
    }
}


#[derive(Debug)]
pub struct Board {
    board: Vec<Field>,
    placed: usize,
}

impl Board {
    pub fn create_board(board_vec: Vec<Vec<i8>>) -> Self {
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

    pub fn new(board_str: &str) -> Result<Self, SudokuErrors> {
        if board_str.len() != 81 {
            return Err(InvalidBoardConfig);
        }
        let mut board: Vec<Field> = Vec::with_capacity(81);
        let mut placed: usize = 0;

        for c in board_str.chars() {
            let value = Field::try_from(c)?;
            match value {
                EMPTY => board.push(EMPTY),
                _ => {
                    placed += 1;
                    board.push(value)
                }
            }
        }
        Ok(Board {
            board,
            placed,
        })
    }

    pub fn to_string(&self) -> String {
        let mut output = String::with_capacity(81);
        for field in &self.board {
            output.push(field.clone().into());
        }
        output
    }
}


impl Board {
    pub fn solve_board(&self) -> Result<Board, SudokuErrors> {
        let board = self.board.clone();
        let placed = self.placed.clone();
        let mut board = Board {
            board,
            placed,
        };
        if self.solve_board_rec(&mut board) {
            return Ok(board);
        }
        Err(Unsolvable)
    }

    fn solve_board_rec(&self, board: &mut Board) -> bool {
        if board.placed == 81 {
            return true;
        }

        // Find the first empty cell
        let mut empty_index = None;
        for (i, &field) in board.board.iter().enumerate() {
            if field == EMPTY {
                empty_index = Some(i);
                break;
            }
        }
        //the cell has to exist because of the guard condition self.placed = 81
        let empty_index = empty_index.expect("The board is not valid");
        for i in 1..=9 {
            board.board[empty_index] = GUESS(i);
            board.placed += 1;
            if board.check_valid(empty_index) {
                if self.solve_board_rec(board) {
                    return true;
                }
            }
            board.board[empty_index] = EMPTY;
            board.placed -= 1;
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


#[cfg(test)]
mod test {
    use crate::Board;

    #[test]
    fn test_creation() {
        let board_str = "070004130000207006005013020001002000002190057003045802010378260367000580809001070";
        dbg!(board_str.len());
        Board::new(board_str).unwrap();
    }

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

        let board = Board::create_board(board_pos);
        //dbg!(&board);
        //board.board[0] = Field::GUESS(8);
        //println!("{}", board.check_valid(0));
        board.solve_board().unwrap();
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
            vec![0, 0, 0, 0, 0, 0, 2, 0, 0],
        ];

        let board = Board::create_board(board_pos);
        //dbg!(&board);
        //board.board[0] = Field::GUESS(8);
        //println!("{}", board.check_valid(0));
        board.solve_board().unwrap();

    }
}