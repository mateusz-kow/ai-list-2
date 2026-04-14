use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

static ZOBRIST: OnceLock<Vec<Vec<u64>>> = OnceLock::new();

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Player { WHITE, BLACK }

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::WHITE => Player::BLACK,
            Player::BLACK => Player::WHITE,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Field { OCCUPIED(Player), EMPTY }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub fields: Vec<Vec<Field>>,
    pub hash: u64,
    pub last_move_from: Option<(usize, usize)>,
    pub m: usize,
    pub n: usize,
}

impl Board {
    fn get_zobrist_table() -> &'static Vec<Vec<u64>> {
        ZOBRIST.get_or_init(|| {
            let mut v = vec![vec![0u64; 2]; 1024];
            for i in 0..1024 {
                for j in 0..2 { v[i][j] = rand::random::<u64>(); }
            }
            v
        })
    }

    pub fn default_start(m: usize, n: usize) -> Board {
        let mut fields = vec![vec![Field::EMPTY; n]; m];
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == 1 {
                    fields[i][j] = Field::OCCUPIED(Player::WHITE);
                } else if i == m - 1 || i == m - 2 {
                    fields[i][j] = Field::OCCUPIED(Player::BLACK);
                }
            }
        }
        Self::new(m, n, fields)
    }

    pub fn new(m: usize, n: usize, fields: Vec<Vec<Field>>) -> Board {
        let mut h = 0u64;
        let table = Self::get_zobrist_table();
        for i in 0..m {
            for j in 0..n {
                let idx = i * n + j;
                if let Field::OCCUPIED(p) = fields[i][j] {
                    let p_idx = if p == Player::WHITE { 0 } else { 1 };
                    h ^= table[idx][p_idx];
                }
            }
        }
        Board { fields, hash: h, last_move_from: None, m, n }
    }

    pub fn get_possible_moves(&self, player: Player) -> Vec<Board> {
        let mut boards = Vec::new();
        for i in 0..self.m {
            for j in 0..self.n {
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

        let direction: isize = if player == Player::WHITE { 1 } else { -1 };

        let target_i = i as isize + direction;

        if target_i < 0 || target_i >= self.m as isize { return moves; }
        let ti = target_i as usize;


        for dj in &[-1, 0, 1] {
            let target_j = j as isize + dj;
            if target_j < 0 || target_j >= self.n as isize { continue; }

            let tj = target_j as usize;
            let target_field = self.fields[ti][tj];

            if *dj == 0 {
                if target_field == Field::EMPTY {
                    moves.push(self.create_move(i, j, ti, tj));
                }
            } else {

                if target_field != Field::OCCUPIED(player) {
                    moves.push(self.create_move(i, j, ti, tj));
                }
            }
        }
        moves
    }

    fn create_move(&self, f_i: usize, f_j: usize, t_i: usize, t_j: usize) -> Board {
        let mut next_fields = self.fields.clone();
        let table = Self::get_zobrist_table();
        let player = match self.fields[f_i][f_j] {
            Field::OCCUPIED(p) => p,
            _ => unreachable!(),
        };
        let p_idx = if player == Player::WHITE { 0 } else { 1 };
        let opp_idx = 1 - p_idx;

        let mut next_hash = self.hash;

        next_hash ^= table[f_i * self.n + f_j][p_idx];

        if let Field::OCCUPIED(_) = self.fields[t_i][t_j] {
            next_hash ^= table[t_i * self.n + t_j][opp_idx];
        }

        next_hash ^= table[t_i * self.n + t_j][p_idx];

        next_fields[t_i][t_j] = next_fields[f_i][f_j];
        next_fields[f_i][f_j] = Field::EMPTY;

        Board {
            fields: next_fields,
            hash: next_hash,
            last_move_from: Some((f_i, f_j)),
            m: self.m,
            n: self.n,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.m {
            for j in 0..self.n {
                let symbol = match self.fields[i][j] {
                    Field::EMPTY => {
                        if Some((i, j)) == self.last_move_from { "o" } else { "_" }
                    },
                    Field::OCCUPIED(Player::WHITE) => "W",
                    Field::OCCUPIED(Player::BLACK) => "B",
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}