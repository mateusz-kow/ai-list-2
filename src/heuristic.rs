use crate::board::{Board, Field, Player};

pub trait Heuristic {
    fn eval_state(&self, player: &Player, board: &Board) -> i8;
}

pub struct OccupiedSquaresCountHeuristic {}


impl Heuristic for OccupiedSquaresCountHeuristic {
    // Only takes player's and enemies occupied squares count into account
    fn eval_state(&self, player: &Player, board: &Board) -> i8 {
        let mut result = 0;

        for row in &board.fields {
            for field in row {
                result += match field {
                    Field::OCCUPIED(p) if p == player => 1,
                    Field::OCCUPIED(_) => -1,
                    Field::EMPTY => 0,
                }
            }
        }

        result
    }
}