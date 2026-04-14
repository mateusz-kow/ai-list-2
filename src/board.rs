use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Player { WHITE, BLACK }

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::WHITE => Player::BLACK,
            Player::BLACK => Player::WHITE,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Field { OCCUPIED(Player), EMPTY }

#[derive(Debug, Clone)]
pub struct Board {
    pub fields: Vec<Vec<Field>>,
    pub last_move_from: Option<(usize, usize)>,
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
        Board { fields, last_move_from: None }
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

        let nj = j as isize + direction;
        if nj < 0 || nj >= width { return moves; }

        for &ni_off in &[-1, 0, 1] {
            let ni = i as isize + ni_off;
            if ni < 0 || ni >= height { continue; }
            let target = self.fields[ni as usize][nj as usize];

            if ni_off == 0 {
                if target == Field::EMPTY {
                    moves.push(self.create_move(i, j, ni as usize, nj as usize));
                }
            } else {
                if target != Field::OCCUPIED(player) {
                    moves.push(self.create_move(i, j, ni as usize, nj as usize));
                }
            }
        }
        moves
    }

    fn create_move(&self, f_i: usize, f_j: usize, t_i: usize, t_j: usize) -> Board {
        let mut next_fields = self.fields.clone();
        next_fields[t_i][t_j] = next_fields[f_i][f_j];
        next_fields[f_i][f_j] = Field::EMPTY;
        Board { fields: next_fields, last_move_from: Some((f_i, f_j)) }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.fields.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                if self.last_move_from == Some((i, j)) {
                    write!(f, "o ")?;
                } else {
                    let sym = match field {
                        Field::EMPTY => "_",
                        Field::OCCUPIED(Player::WHITE) => "W",
                        Field::OCCUPIED(Player::BLACK) => "B",
                    };
                    write!(f, "{} ", sym)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}