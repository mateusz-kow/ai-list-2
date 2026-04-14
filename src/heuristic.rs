use crate::board::{Board, Field, Player};

pub trait Heuristic: Send + Sync {
    fn eval_state(&self, board: &Board) -> i32;
}

pub struct MaterialHeuristic;
impl Heuristic for MaterialHeuristic {
    fn eval_state(&self, board: &Board) -> i32 {
        let mut res = 0;
        for row in &board.fields {
            for field in row {
                match field {
                    Field::OCCUPIED(Player::WHITE) => res += 100,
                    Field::OCCUPIED(Player::BLACK) => res -= 100,
                    _ => {}
                }
            }
        }
        res
    }
}

pub struct ProgressHeuristic;
impl Heuristic for ProgressHeuristic {
    fn eval_state(&self, board: &Board) -> i32 {
        let mut res = 0;
        for (i, row) in board.fields.iter().enumerate() {
            for field in row {
                if let Field::OCCUPIED(p) = field {
                    let dist = if *p == Player::WHITE { i } else { (board.m - 1) - i };
                    let val = 10 + (dist * dist) as i32;

                    let win_bonus = if dist == board.m - 1 { 5000 } else { 0 };

                    res += if *p == Player::WHITE { val + win_bonus } else { -(val + win_bonus) };
                }
            }
        }
        res
    }
}

pub struct BreakthroughHeuristic;
impl Heuristic for BreakthroughHeuristic {
    fn eval_state(&self, board: &Board) -> i32 {
        let mut score = 0;
        for (i, row) in board.fields.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                if let Field::OCCUPIED(p) = field {
                    let is_white = *p == Player::WHITE;
                    let dist = if is_white { i } else { (board.m - 1) - i };

                    let mut p_score = 10 + (dist * 10) as i32;

                    if is_white && i > 0 {
                        if j > 0 && board.fields[i-1][j-1] == Field::OCCUPIED(Player::WHITE) { p_score += 5; }
                        if j < board.n-1 && board.fields[i-1][j+1] == Field::OCCUPIED(Player::WHITE) { p_score += 5; }
                    } else if !is_white && i < board.m - 1 {
                        if j > 0 && board.fields[i+1][j-1] == Field::OCCUPIED(Player::BLACK) { p_score += 5; }
                        if j < board.n-1 && board.fields[i+1][j+1] == Field::OCCUPIED(Player::BLACK) { p_score += 5; }
                    }

                    if dist == board.m - 1 { p_score += 10000; }
                    score += if is_white { p_score } else { -p_score };
                }
            }
        }
        score
    }
}

pub fn get_heuristic(idx: usize) -> Box<dyn Heuristic> {
    match idx {
        1 => Box::new(MaterialHeuristic),
        2 => Box::new(ProgressHeuristic),
        _ => Box::new(BreakthroughHeuristic),
    }
}