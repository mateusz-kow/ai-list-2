mod tree;
mod heuristic;
mod board;
mod engine;

use tree::TreeNode;
use crate::board::{Board, Field, Player};
use crate::heuristic::{OccupiedSquaresCountHeuristic, Heuristic};

fn main() {
    let depth: usize = 5;
    let m: usize = 8;
    let n: usize = 8;

    let mut board = Board::new(m, n);
    let heuristic = OccupiedSquaresCountHeuristic {};
    let mut current_player = Player::WHITE;

    println!("Initial Board:\n{}", board);

    for i in 0..100 {
        println!("--- Turn {}: {:?} ---", i, current_player);

        let mut root = TreeNode::new(board.clone(), current_player);
        root.compute_minimax(depth, true, &heuristic, current_player);

        if let Some(best_node) = root.children.iter().max_by_key(|c| c.value) {
            board = best_node.board.clone();
        } else {
            println!("No moves left for {:?}", current_player);
            break;
        }

        println!("{}", board);

        if let Some(winner) = determine_winner(&board) {
            println!("GAME OVER! {:?} won!", winner);
            break;
        }

        current_player = current_player.opponent();
    }
}

fn determine_winner(board: &Board) -> Option<Player> {
    let width = board.fields[0].len();
    for row in &board.fields {
        if row[width - 1] == Field::OCCUPIED(Player::WHITE) {
            return Some(Player::WHITE);
        }

        if row[0] == Field::OCCUPIED(Player::BLACK) {
            return Some(Player::BLACK);
        }
    }

    None
}
