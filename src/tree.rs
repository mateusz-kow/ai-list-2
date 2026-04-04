use crate::board::{Board, Player};
use crate::heuristic::{Heuristic};

#[derive(Debug, Clone)]
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

    fn recalculate_values(&mut self, depth: usize, maximizing: bool) -> i8 {
        if depth == 0 || self.children.is_none() {
            return self.value;
        }

        let children = self.children.as_mut().unwrap();

        if maximizing {
            let mut best = i8::MIN;

            for child in children {
                let val = child.recalculate_values(depth - 1, false);
                best = best.max(val);
            }

            self.value = best;
        } else {
            let mut best = i8::MAX;

            for child in children {
                let val = child.recalculate_values(depth - 1, true);
                best = best.min(val);
            }

            self.value = best;
        }

        self.value
    }

    pub fn minmax(&mut self, depth: usize, maximizing: bool) -> Option<&TreeNode> {
        self.recalculate_values(depth, maximizing);

        let children = self.children.as_ref()?;

        if maximizing {
            children.iter().max_by_key(|child| child.value).map(|c| c.as_ref())
        } else {
            children.iter().min_by_key(|child| child.value).map(|c| c.as_ref())
        }
    }


    pub fn expand(&mut self, depth: usize, heuristic: &dyn Heuristic) {
        if depth == 0 { return; }

        let next_moves: Vec<Board> = self.board.get_possible_moves(&self.player);

        let mut children = Vec::new();
        for next_board in next_moves {
            let eval = heuristic.eval_state(&self.player, &next_board);

            let mut child = Box::new(TreeNode::new(next_board, eval, self.player));

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
