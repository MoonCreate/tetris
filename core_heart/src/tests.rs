use crate::{game::TetrisGame, tetromino::{Tetromino, TetrominoDirection}};

#[test]
fn must() {
    let tetromino = Tetromino::I;
    tetromino.print(TetrominoDirection::Up);
    println!("");
    tetromino.print(TetrominoDirection::Right);
    println!("");
    tetromino.print(TetrominoDirection::Down);
    println!("");
    tetromino.print(TetrominoDirection::Left);
}

#[test]
fn create() {
    let game = TetrisGame::new();
}