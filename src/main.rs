mod tree;
mod heuristic;
mod board;
mod engine;

use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use tree::TreeNode;
use crate::board::{Board, Player};
use crate::heuristic::OccupiedSquaresCountHeuristic;

fn main() {
    let depth: usize = 5;
    let strategy = 'B';
    let m: usize = 8;
    let n: usize = 8;

    let turn = Player::WHITE;
    let opponent = Player::BLACK;

    let mut board = Board::new(m, n);
    let heuristic = OccupiedSquaresCountHeuristic {};
    let mut current_node = TreeNode::new(board, 0, turn);
    
    current_node.expand(depth, &heuristic);
    print!("{:?}", current_node)
}
