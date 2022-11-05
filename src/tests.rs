use crate::env::{Connect4, Player};

#[test]
fn win_condition() {
    let mut board = Connect4::default();
    let overall_win ;
    loop {
        board.add(0);
        if let Some(win) = board.check_win() {
            println!("{}{:?} won", board, win);
            overall_win = Some(win);
            break;
        }

        board.add(1);
        if let Some(win) = board.check_win() {
            println!("{}{:?} won", board, win);
            overall_win = Some(win);
            break;
        }
    }
    assert!(matches!(overall_win, Some(Player::RED)));
}
