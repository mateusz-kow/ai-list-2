use crate::board::{Board, Player};
use crate::heuristic::{Heuristic};

#[derive(Debug)]
pub struct TreeNode {
    board: Board,
    player: Player,
    value: i8,
    children: Option<Vec<Box<TreeNode>>>,
}

impl TreeNode {
    pub fn new(board: Board, value: i8, player: Player) -> TreeNode {
        TreeNode { board, value, player, children : None }
    }

    pub fn expand(&mut self, depth: usize, heuristic: &dyn Heuristic) {
        if depth == 0 { return; }

        let next_moves: Vec<Board> = self.board.get_possible_moves(&self.player);
        let next_player = self.player.opponent();

        let mut children = Vec::new();
        for next_board in next_moves {
            let eval = heuristic.eval_state(&self.player, &next_board);

            let mut child = Box::new(TreeNode::new(next_board, eval, next_player));

            child.expand(depth - 1, heuristic);

            children.push(child);
        }

        self.children = Some(children);
    }

    pub fn get_children(&self) -> &[Box<TreeNode>] {
        match &self.children {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn set_children(&mut self, children: Vec<Box<TreeNode>>) {
        self.children = Some(children);
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_value(&self) -> i8 {
        self.value
    }
}
