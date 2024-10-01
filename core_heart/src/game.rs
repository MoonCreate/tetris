use wasm_bindgen::prelude::*;

use crate::tetromino::{Tetromino, TetrominoDirection};

#[wasm_bindgen]
pub struct TetrisGame {
    board: [[u8; 10]; 20],
    pub current_tetromino: Tetromino,
    pub current_tetromino_direction: TetrominoDirection,
    pub current_x: u8,
    pub current_y: u8,
}

#[wasm_bindgen]
impl TetrisGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            board: [[0u8; 10]; 20],
            current_tetromino: Tetromino::T,
            current_tetromino_direction: TetrominoDirection::Up,
            current_x: 0u8,
            current_y: 0u8,
        }
    }

    pub fn print(&self) -> String {
        let tetromino = self.current_tetromino.get_array(self.current_tetromino_direction);
        let mut board = self.board;

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

                board[place_y][place_x] = tetromino[i][j];
            }
        }
        format!("{:?}", board)
    }

    pub fn move_left(&mut self) {
        if self.current_x == 0 { return }
        self.current_x -= 1;
    }

    pub fn move_right(&mut self) {
        if self.current_x == 9 { return }
        self.current_x += 1;
    }

    pub fn move_down(&mut self) {
        if self.current_y == 19 { return }
        self.current_y += 1;
    }

    pub fn move_up(&mut self) {
        if self.current_y == 0 { return }
        self.current_y -=1;
    }

    pub fn change(&mut self, tetromino: Tetromino) {
        self.current_tetromino = tetromino;
    }

    pub fn rotate(&mut self) {
        self.current_tetromino_direction.rotate_clockwise();
    }
}