use crate::board::{Board, Field, Player};

pub trait Heuristic {
    fn eval_state(&self, board: &Board) -> i32;
}

pub struct OccupiedSquaresCountHeuristic;
impl Heuristic for OccupiedSquaresCountHeuristic {
    fn eval_state(&self, board: &Board) -> i32 {
        let mut res = 0;
        for row in &board.fields {
            for field in row {
                match field {
                    Field::OCCUPIED(Player::WHITE) => res += 10,
                    Field::OCCUPIED(Player::BLACK) => res -= 10,
                    _ => {}
                }
            }
        }
        res
    }
}

pub struct SumDistanceHeuristic;
impl Heuristic for SumDistanceHeuristic {
    fn eval_state(&self, board: &Board) -> i32 {
        let mut res = 0;
        let width = board.fields[0].len();
        for row in &board.fields {
            for (y, field) in row.iter().enumerate() {
                if let Field::OCCUPIED(p) = field {
                    let val = 2_i32.pow(if *p == Player::WHITE { y } else { width - 1 - y } as u32);
                    res += if *p == Player::WHITE { val } else { -val };
                }
            }
        }
        res
    }
}

pub struct MaxDistanceHeuristic;
impl Heuristic for MaxDistanceHeuristic {
    fn eval_state(&self, board: &Board) -> i32 {
        let mut w_max = 0;
        let mut b_max = 0;
        let width = board.fields[0].len();
        for row in &board.fields {
            for (y, field) in row.iter().enumerate() {
                if let Field::OCCUPIED(p) = field {
                    let val = 2_i32.pow(if *p == Player::WHITE { y } else { width - 1 - y } as u32);
                    if *p == Player::WHITE { w_max = w_max.max(val); } else { b_max = b_max.max(val); }
                }
            }
        }
        w_max - b_max
    }
}