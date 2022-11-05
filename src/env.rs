use std::{fmt::{Display, Write}, ops::{Index, IndexMut}};

use tensorflow::Tensor;

const COLS: u8 = 7;
const ROWS: u8 = 6;

#[allow(non_snake_case)]
pub mod Slot {
    use std::fmt::Write;

    pub const EMPTY: u8 = 0;
    pub const RED: u8 = 1;
    pub const YELLOW: u8 = 2;

    pub fn fmt(s: &u8, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match *s {
            EMPTY => ' ',
            RED => 'R',
            YELLOW => 'Y',
            _ => panic!(),
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

impl From<Player> for u8 {
    fn from(value: Player) -> Self {
        match value {
            Player::RED => Slot::RED,
            Player::YELLOW => Slot::YELLOW,
        }
    }
}

/// Current state of a Connect-4 game.
#[derive(Clone, Default, Debug)]
pub struct Connect4 {
    pub board: Connect4Board,
    pub current_player: Player,
}

#[derive(Clone, Debug)]
pub struct Connect4Board(Tensor<u8>);

impl Default for Connect4Board {
    fn default() -> Self {
        Self(Tensor::new(&[COLS as u64, ROWS as u64]))
    }
}

impl Index<(u8, u8)> for Connect4Board {
    type Output = u8;

    fn index(&self, index: (u8, u8)) -> &Self::Output {
        &self.0[self.0.get_index(&[index.0 as u64, index.1 as u64])]
    }
}

impl IndexMut<(u8, u8)> for Connect4Board {
    fn index_mut(&mut self, index: (u8, u8)) -> &mut Self::Output {
        let idx = self.0.get_index(&[index.0 as u64, index.1 as u64]);
        &mut self.0[idx]
    }
}

impl Connect4 {
    pub fn add(&mut self, col: u8) {
        assert!((0..COLS).contains(&col));
        for j in (0..ROWS).rev() {
            let slot = &mut self.board[(col, j)];
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
                if self.board[(col as u8, row as u8)] != slot {
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
        for j in 0..ROWS {
            f.write_char('|')?;
            for i in 0..COLS {
                Slot::fmt(&self.board[(i, j)], f)?;
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
