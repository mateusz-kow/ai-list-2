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
                    res += if *p == Player::WHITE { val } else { -val };
                }
            }
        }
        res
    }
}

pub struct BreakthroughHeuristic;
impl Heuristic for BreakthroughHeuristic {
    fn eval_state(&self, board: &Board) -> i32 {
        let mut w_score = 0;
        let mut b_score = 0;
        for (i, row) in board.fields.iter().enumerate() {
            for field in row {
                if let Field::OCCUPIED(p) = field {
                    if *p == Player::WHITE {
                        w_score += 10 + (i * 5) as i32;
                    } else {
                        let dist = (board.m - 1) - i;
                        b_score += 10 + (dist * 5) as i32;
                    }
                }
            }
        }
        w_score - b_score
    }
}

pub fn get_heuristic(idx: usize) -> Box<dyn Heuristic> {
    match idx {
        1 => Box::new(MaterialHeuristic),
        2 => Box::new(ProgressHeuristic),
        _ => Box::new(BreakthroughHeuristic),
    }
}