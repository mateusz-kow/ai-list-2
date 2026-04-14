mod board;
mod heuristic;
mod tree;

use std::io::{self, BufRead};
use std::time::Instant;
use crate::board::{Board, Player, Field};
use crate::heuristic::*;
use crate::tree::AlphaBeta;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    println!("Podaj m, n, d (np. 8 8 5):");
    let line = input.next().unwrap().unwrap();
    let vals: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (m, n, d) = (vals[0], vals[1], vals[2]);

    println!("Wybierz heurystyki (1: PieceCount, 2: SumDist, 3: MaxDist) dla Gracza 1 i 2:");
    let line = input.next().unwrap().unwrap();
    let h_ids: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let h1 = get_h(h_ids[0]);
    let h2 = get_h(h_ids[1]);

    let mut board = Board::new(m, n);
    let mut current_player = Player::WHITE;
    let mut turns = 0;

    println!("Start:\n{}", board);

    while turns < 200 {
        let start = Instant::now();
        let mut nodes = 0;
        let h = if current_player == Player::WHITE { &*h1 } else { &*h2 };

        let next_board = AlphaBeta::get_best_move(&board, d, current_player, h, &mut nodes);

        let elapsed = start.elapsed();
        eprintln!("Gracz {:?} | Węzły: {} | Czas: {:?}", current_player, nodes, elapsed);

        if let Some(nb) = next_board {
            board = nb;
        } else {
            println!("Brak ruchów dla {:?}.", current_player);
            break;
        }

        turns += 1;
        println!("Tura {}:\n{}", turns, board);

        if let Some(winner) = check_winner(&board) {
            println!("KONIEC! Wygrał: {:?}. Rundy: {}", winner, (turns + 1) / 2);
            break;
        }
        current_player = current_player.opponent();
    }
}

fn get_h(id: usize) -> Box<dyn Heuristic> {
    match id {
        1 => Box::new(OccupiedSquaresCountHeuristic),
        2 => Box::new(SumDistanceHeuristic),
        _ => Box::new(MaxDistanceHeuristic),
    }
}

fn check_winner(board: &Board) -> Option<Player> {
    let width = board.fields[0].len();
    for row in &board.fields {
        if row[width - 1] == Field::OCCUPIED(Player::WHITE) { return Some(Player::WHITE); }
        if row[0] == Field::OCCUPIED(Player::BLACK) { return Some(Player::BLACK); }
    }
    None
}