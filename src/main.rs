mod tree;
mod heuristic;
mod board;
mod engine;

use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use tree::TreeNode;
use crate::board::{Board, Field, Player};
use crate::heuristic::OccupiedSquaresCountHeuristic;

fn main() {
    let depth: usize = 5;
    let m: usize = 8;
    let n: usize = 8;

    let player = Player::WHITE;

    let starting_board = Board::new(m, n);
    let heuristic = OccupiedSquaresCountHeuristic {};
    let mut current_node = TreeNode::new(starting_board, 0, player);
    let mut maximizing = true;

    for i in 0..100 {
        println!("Iteration {}", i);

        current_node.expand(depth, &heuristic);
        current_node = current_node.minmax(depth, maximizing).unwrap().clone();

        println!("Current value: {}", current_node.get_value());

        maximizing = !maximizing;
        let current_board = current_node.get_board();
        let winner = determine_winner(current_board);

        match winner {
            Some(winner) => {
                println!("{:?} won!", winner);
                println!("Board:\n{}", current_board);
                break
            },
            None => {}
        }
    }
}

fn determine_winner(board: &Board) -> Option<Player> {
    let height = board.fields.len();
    let width = board.fields[0].len();

    for i in 0..height {
        if board.fields[i][0] == Field::OCCUPIED(Player::BLACK) {
            return Some(Player::BLACK);
        }
        if board.fields[i][width-1] == Field::OCCUPIED(Player::BLACK) {
            return Some(Player::BLACK);
        }
    }

    None
}
