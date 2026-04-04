use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum Player {
    #[serde(rename = "W")]
    WHITE,
    #[serde(rename = "B")]
    BLACK,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::WHITE => Player::BLACK,
            Player::BLACK => Player::WHITE,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[serde(untagged)]
pub enum Field {
    OCCUPIED(Player),
    #[serde(rename = "_")]
    EMPTY,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub fields: Vec<Vec<Field>>,
}

impl Board {
    pub fn new(m: usize, n: usize) -> Board {
        let mut fields = vec![vec![Field::EMPTY; n]; m];
        for i in 0..m {
            fields[i][0] = Field::OCCUPIED(Player::WHITE);
            fields[i][1] = Field::OCCUPIED(Player::WHITE);
            fields[i][n - 2] = Field::OCCUPIED(Player::BLACK);
            fields[i][n - 1] = Field::OCCUPIED(Player::BLACK);
        }
        Board { fields }
    }

    pub fn get_possible_moves(&self, player: Player) -> Vec<Board> {
        let mut boards = Vec::new();
        for i in 0..self.fields.len() {
            for j in 0..self.fields[0].len() {
                if let Field::OCCUPIED(p) = self.fields[i][j] {
                    if p == player {
                        boards.append(&mut self.get_moves_for_piece(i, j, player));
                    }
                }
            }
        }
        boards
    }

    fn get_moves_for_piece(&self, i: usize, j: usize, player: Player) -> Vec<Board> {
        let mut moves = Vec::new();
        let height = self.fields.len() as isize;
        let width = self.fields[0].len() as isize;
        let direction: isize = if player == Player::WHITE { 1 } else { -1 };

        let next_j = j as isize + direction;
        if next_j < 0 || next_j >= width { return moves; }

        let nj = next_j as usize;
        let rows = [i as isize - 1, i as isize, i as isize + 1];

        for &ni_isize in &rows {
            if ni_isize < 0 || ni_isize >= height { continue; }
            let ni = ni_isize as usize;
            let target = self.fields[ni][nj];

            if ni == i {
                if target == Field::EMPTY {
                    moves.push(self.create_move(i, j, ni, nj));
                }
            } else {
                if target != Field::OCCUPIED(player) {
                    moves.push(self.create_move(i, j, ni, nj));
                }
            }
        }
        moves
    }

    fn create_move(&self, f_i: usize, f_j: usize, t_i: usize, t_j: usize) -> Board {
        let mut next_fields = self.fields.clone();
        next_fields[t_i][t_j] = next_fields[f_i][f_j];
        next_fields[f_i][f_j] = Field::EMPTY;
        Board { fields: next_fields }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.fields {
            for field in row {
                let sym = match field {
                    Field::EMPTY => "_",
                    Field::OCCUPIED(Player::WHITE) => "W",
                    Field::OCCUPIED(Player::BLACK) => "B",
                };
                write!(f, "{} ", sym)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}