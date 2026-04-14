use crate::board::{Board, Player};
use crate::heuristic::Heuristic;

pub struct AlphaBeta;

impl AlphaBeta {
    pub fn get_best_move(board: &Board, depth: usize, player: Player, heuristic: &dyn Heuristic, nodes: &mut u64) -> Option<Board> {
        let moves = board.get_possible_moves(player);
        let mut best_val = if player == Player::WHITE { i32::MIN } else { i32::MAX };
        let mut best_board = None;

        for m in moves {
            let val = Self::search(&m, depth - 1, i32::MIN, i32::MAX, player.opponent(), heuristic, nodes);
            if player == Player::WHITE {
                if val >= best_val { best_val = val; best_board = Some(m); }
            } else {
                if val <= best_val { best_val = val; best_board = Some(m); }
            }
        }
        best_board
    }

    fn search(board: &Board, depth: usize, mut alpha: i32, mut beta: i32, player: Player, heuristic: &dyn Heuristic, nodes: &mut u64) -> i32 {
        *nodes += 1;
        if depth == 0 { return heuristic.eval_state(board); }

        let moves = board.get_possible_moves(player);
        if moves.is_empty() { return heuristic.eval_state(board); }

        if player == Player::WHITE {
            let mut max_eval = i32::MIN;
            for m in moves {
                let eval = Self::search(&m, depth - 1, alpha, beta, Player::BLACK, heuristic, nodes);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha { break; }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for m in moves {
                let eval = Self::search(&m, depth - 1, alpha, beta, Player::WHITE, heuristic, nodes);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha { break; }
            }
            min_eval
        }
    }
}