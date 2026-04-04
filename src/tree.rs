use crate::board::{Board, Player};
use crate::heuristic::Heuristic;

#[derive(Clone)]
pub struct TreeNode {
    pub board: Board,
    pub player_to_move: Player,
    pub value: i32,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(board: Board, player_to_move: Player) -> Self {
        Self { board, player_to_move, value: 0, children: Vec::new() }
    }

    // Standard Minimax with depth limit
    pub fn compute_minimax(&mut self, depth: usize, maximizing: bool, heuristic: &dyn Heuristic, original_player: Player) -> i32 {
        if depth == 0 {
            self.value = heuristic.eval_state(&original_player, &self.board) as i32;
            return self.value;
        }

        let moves = self.board.get_possible_moves(self.player_to_move);
        if moves.is_empty() {
            self.value = heuristic.eval_state(&original_player, &self.board) as i32;
            return self.value;
        }

        let mut children = Vec::new();
        for m in moves {
            let mut child = TreeNode::new(m, self.player_to_move.opponent());
            child.compute_minimax(depth - 1, !maximizing, heuristic, original_player);
            children.push(child);
        }

        let res = if maximizing {
            children.iter().map(|c| c.value).max().unwrap_or(i32::MIN)
        } else {
            children.iter().map(|c| c.value).min().unwrap_or(i32::MAX)
        };

        self.children = children;
        self.value = res;
        res
    }
}
