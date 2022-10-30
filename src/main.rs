#![doc = include_str!("../README.md")]

use crate::env::Connect4;

mod env;

fn main() {
    println!("preparing to take over the world...\n");

    let mut board = Connect4::default();
    loop {
        board.add(0);
        if let Some(win) = board.check_win() {
            println!("{}{:?} won", board, win);
            break;
        }

        board.add(1);
        if let Some(win) = board.check_win() {
            println!("{}{:?} won", board, win);
            break;
        }
    }
}
