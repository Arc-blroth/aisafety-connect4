use std::fmt::{Display, Write};

const COLS: u8 = 7;
const ROWS: u8 = 6;

#[repr(u8)]
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Slot {
    #[default]
    EMPTY = 0,
    RED = 1,
    YELLOW = 2,
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Slot::EMPTY => ' ',
            Slot::RED => 'R',
            Slot::YELLOW => 'Y',
        })
    }
}

#[repr(u8)]
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Player {
    #[default]
    RED,
    YELLOW,
}

impl From<Player> for Slot {
    fn from(value: Player) -> Self {
        match value {
            Player::RED => Slot::RED,
            Player::YELLOW => Slot::YELLOW,
        }
    }
}

/// Current state of a Connect-4 game.
#[derive(Default, PartialEq, Eq, Clone)]
pub struct Connect4 {
    pub board: [[Slot; ROWS as usize]; COLS as usize],
    pub current_player: Player,
}

impl Connect4 {
    pub fn add(&mut self, col: u8) {
        assert!((0..COLS).contains(&col));
        for j in (0..ROWS).rev() {
            let slot = &mut self.board[col as usize][j as usize];
            if slot == &Slot::EMPTY {
                *slot = self.current_player.into();
                self.current_player = match self.current_player {
                    Player::RED => Player::YELLOW,
                    Player::YELLOW => Player::RED,
                };
                return;
            }
        }
        panic!("col is full!");
    }

    fn check_line(&self, player: Player, pos: (u8, u8), delta: (i8, i8)) -> bool {
        let slot = player.into();
        let mut col = pos.0 as i8;
        let mut row = pos.1 as i8;
        for _ in 0..4 {
            if (0..COLS as i8).contains(&col) && (0..ROWS as i8).contains(&row) {
                if self.board[col as usize][row as usize] != slot {
                    return false;
                }
            } else {
                return false;
            }
            col += delta.0;
            row += delta.1;
        }
        true
    }

    fn check_pos(&self, player: Player, pos: (u8, u8)) -> bool {
        self.check_line(player, pos, (-1, 0))
        || self.check_line(player, pos, (0, 1))
        || self.check_line(player, pos, (0, -1))
        || self.check_line(player, pos, (1, 1))
        || self.check_line(player, pos, (-1, -1))
        || self.check_line(player, pos, (-1, 1))
        || self.check_line(player, pos, (1, -1))
    }

    pub fn check_win(&self) -> Option<Player> {
        for i in 0..COLS {
            for j in 0..ROWS {
                if self.check_pos(Player::RED, (i, j)) {
                    return Some(Player::RED);
                }
                if self.check_pos(Player::YELLOW, (i, j)) {
                    return Some(Player::YELLOW);
                }
            }
        }
        None
    }
}

impl Display for Connect4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..ROWS as usize {
            f.write_char('|')?;
            for i in 0..COLS as usize {
                self.board[i][j].fmt(f)?;
            }
            f.write_char('|')?;
            f.write_char('\n')?;
        }
        for _ in 0..COLS + 2 {
            f.write_char('=')?;
        }
        f.write_char('\n')
    }
}
