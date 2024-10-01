use wasm_bindgen::prelude::*;

use crate::tetromino::{Tetromino, TetrominoDirection};

#[wasm_bindgen]
pub struct TetrisGame {
    board: [[u8; 10]; 20],
    pub current_tetromino: Tetromino,
    pub current_tetromino_direction: TetrominoDirection,
    pub current_x: i8,
    pub current_y: i8,
}

#[wasm_bindgen]
impl TetrisGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            board: [[0u8; 10]; 20],
            current_tetromino: Tetromino::T,
            current_tetromino_direction: TetrominoDirection::Up,
            current_x: 0,
            current_y: 0,
        }
    }

    fn is_occupied(&self) -> bool {
        let tetromino = self.current_tetromino.get_array(self.current_tetromino_direction);
        let board = self.board;
        for y in 0..4 {
            let place_y = self.current_y + y;
            for x in 0..4 {
                let place_x = self.current_x + x;
                if tetromino[y as usize][x as usize] == 1 {
                    // if piece out of range
                    if place_x < 0 || place_y < 0 || place_x >= 10 || place_y >= 20 {
                        return true;
                    }
                    // if occupied
                    if board[place_y as usize][place_x as usize] == 1 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn print(&mut self) -> String {
        let tetromino = self.current_tetromino.get_array(self.current_tetromino_direction);
        let mut board = self.board;
        let ghost = self.get_ghost_position();

        for i in 0usize..4 {
            let place_y = self.current_y as usize + i;
            let ghost_y = ghost.1 as usize + i;
            if place_y > 19 {
                continue
            };
            for j in 0usize..4 {
                let place_x = self.current_x as usize + j;
                let ghost_x = ghost.0 as usize + j;

                if place_x > 9 || tetromino[i][j] == 0 {
                    continue
                };

                board[ghost_y][ghost_x] = 2;
                board[place_y][place_x] = tetromino[i][j];
            }
        }
        format!("{:?}", board)
    }

    pub fn move_left(&mut self) -> bool {
        self.current_x -= 1;
        if self.is_occupied() {
            self.current_x += 1;
            return false;
        }
        true
    }

    pub fn move_right(&mut self) -> bool {
        self.current_x += 1;
        if self.is_occupied() {
            self.current_x -= 1;
            return false;
        }
        true
    }

    pub fn move_down(&mut self) -> bool {
        self.current_y += 1;
        if self.is_occupied() {
            self.current_y -= 1;
            return false;
        }
        true
    }

    pub fn move_up(&mut self) -> bool {
        self.current_y -=1;
        if self.is_occupied() {
            self.current_y += 1;
            return false;
        }
        true
    }

    pub fn change(&mut self, tetromino: Tetromino) {
        self.current_tetromino = tetromino;
    }

    pub fn rotate_clockwise(&mut self) -> bool {
        self.current_tetromino_direction.rotate_clockwise();
        if self.is_occupied() {
            if self.move_up() { return true };
            if self.move_down() { return true };
            if self.move_right() { return true };
            if self.move_left() { return true };
            self.current_tetromino_direction.rotate_counterclockwise();
            return false
        }
        true
    }

    pub fn rotate_counterclockwise(&mut self) -> bool {
        self.current_tetromino_direction.rotate_counterclockwise();
        if self.is_occupied() {
            if self.move_up() { return true };
            if self.move_down() { return true };
            if self.move_right() { return true };
            if self.move_left() { return true };
            self.current_tetromino_direction.rotate_clockwise();
            return false
        }
        true
    }

    pub fn place(&mut self) {
        let tetromino = self.current_tetromino.get_array(self.current_tetromino_direction);
        for i in 0usize..4 {
            let place_y = self.current_y as usize + i;
            if place_y > 19 {
                continue
            };
            for j in 0usize..4 {
                let place_x = self.current_x as usize + j;
                if place_x > 9 || tetromino[i][j] == 0 {
                    continue
                };
                self.board[place_y][place_x] = tetromino[i][j];
            }
        }

        self.current_tetromino = Tetromino::T;
        self.current_tetromino_direction = TetrominoDirection::Up;
        self.current_x = 0;
        self.current_y = 0;
    }

    fn get_ghost_position(&mut self) -> (i8, i8) {
        let prior_y = self.current_y;
        let prior_x = self.current_x;

        while self.move_down() {}

        let result_x = self.current_x;
        let result_y = self.current_y;

        self.current_x = prior_x;
        self.current_y = prior_y;

        (result_x, result_y)
    }
}