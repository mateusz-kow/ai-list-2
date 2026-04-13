use crate::board::{Board, Field, Player};

pub trait Heuristic {
    fn eval_state(&self, turn: &Player, board: &Board) -> i32;
}

pub struct OccupiedSquaresCountHeuristic {}


impl Heuristic for OccupiedSquaresCountHeuristic {
    // Only takes player's and enemies occupied squares count into account
    fn eval_state(&self, turn: &Player, board: &Board) -> i32 {
        let mut result = 0;

        for row in board.fields.iter() {
            for field in row {
                result += match field {
                    Field::OCCUPIED(Player::WHITE) => 1,
                    Field::OCCUPIED(Player::BLACK) => -1,
                    Field::EMPTY => 0,
                }
            }
        }

        result
    }
}

pub struct SumDistanceHeuristic {}

impl Heuristic for SumDistanceHeuristic {
    fn eval_state(&self, turn: &Player, board: &Board) -> i32 {
        let board_len = board.fields[0].len();

        let calc_passed_dist = |player: &Player, y: usize| -> usize {
            match player {
                Player::WHITE => { y }
                Player::BLACK => { board_len - y }
            }
        };

        let calc_value = |player: &Player, y: usize| -> i32 {
            let val = 2_i32.pow(calc_passed_dist(player, y) as u32);
            match player {
                Player::WHITE => { val }
                Player::BLACK => { -val }
            }
        };

        let mut result = 0;

        for row in board.fields.iter() {
            for (y, field) in row.iter().enumerate() {
                result += match field {
                    Field::OCCUPIED(p) => calc_value(p, y),
                    Field::EMPTY => 0,
                }
            }
        }

        result
    }
}
pub struct MaxDistanceHeuristic {}

impl Heuristic for MaxDistanceHeuristic {
    fn eval_state(&self, turn: &Player, board: &Board) -> i32 {
        let board_len = board.fields[0].len();

        let calc_passed_dist = |player: &Player, y: usize| -> usize {
            match player {
                Player::WHITE => { y }
                Player::BLACK => { board_len - y }
            }
        };

        let calc_value = |player: &Player, y: usize| -> i32 {
            let val = 2_i32.pow(calc_passed_dist(player, y) as u32);
            match player {
                Player::WHITE => { val }
                Player::BLACK => { -val }
            }
        };

        let mut white_max_val = 0;
        let mut black_max_val = 0;

        for row in board.fields.iter() {
            for (y, field) in row.iter().enumerate() {
                match field {
                    Field::OCCUPIED(p) => {
                        match p {
                            Player::WHITE => { white_max_val = white_max_val.max(calc_value(p, y)) }
                            Player::BLACK => { black_max_val = black_max_val.max(calc_value(p, y)) }
                        }
                    },
                    Field::EMPTY => {},
                }
            }
        }

        white_max_val - black_max_val
    }
}
