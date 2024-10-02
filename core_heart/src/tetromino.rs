use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum TetrominoDirection {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3
}

impl TetrominoDirection {
    pub fn rotate_clockwise(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

impl Tetromino {
    pub fn get_bytes(&self, direction: TetrominoDirection) -> u16 {
        match self {
            Self::I => match direction {
                TetrominoDirection::Up => 0x4444,
                TetrominoDirection::Right => 0x0f00,
                TetrominoDirection::Down => 0x2222,
                TetrominoDirection::Left => 0x00f0,
            },
            Self::J => match direction {
                TetrominoDirection::Up => 0x44c0,
                TetrominoDirection::Right => 0x8e00,
                TetrominoDirection::Down => 0x6440,
                TetrominoDirection::Left => 0x0e20,
            }
            Self::L => match direction {
                TetrominoDirection::Up => 0x4460,
                TetrominoDirection::Right => 0x0e80,
                TetrominoDirection::Down => 0xc440,
                TetrominoDirection::Left => 0x2e00,
            }
            Self::O => match direction {
                TetrominoDirection::Up => 0xcc00,
                TetrominoDirection::Right => 0xcc00,
                TetrominoDirection::Down => 0xcc00,
                TetrominoDirection::Left => 0xcc00,
            }
            Self::S => match direction {
                TetrominoDirection::Up => 0x6c00,
                TetrominoDirection::Right => 0x4620,
                TetrominoDirection::Down => 0x06c0,
                TetrominoDirection::Left => 0x8c40,
            }
            Self::T => match direction {
                TetrominoDirection::Down => 0x0e40,
                TetrominoDirection::Left => 0x4c40,
                TetrominoDirection::Up => 0x4e00,
                TetrominoDirection::Right => 0x4640,
                
            }
            Self::Z => match direction {
                TetrominoDirection::Up => 0xc600,
                TetrominoDirection::Right => 0x2640,
                TetrominoDirection::Down => 0x0c60,
                TetrominoDirection::Left => 0x4c80,
            }
        }
    }

    pub fn get_array(&self, direction: TetrominoDirection) -> [[u8; 4]; 4] {
        let bytes = self.get_bytes(direction);
        let mut array = [[0u8; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                array[3 - i][3 - j] = (bytes >> (i * 4 + j) & 1) as u8 * self.get_id();
            }
        }
        array
    }

    pub fn get_id(&self) -> u8 {
        match self {
            Self::I => 1,
            Self::J => 2,
            Self::L => 3,
            Self::O => 4,
            Self::S => 5,
            Self::T => 6,
            Self::Z => 7
        }
    }

    pub fn print(&self, direction: TetrominoDirection) {
        let array = self.get_array(direction);
        for i in 0..4 {
            for j in 0..4 {
                print!("{}", array[i][j]);
            }
            println!();
        }
    }
}