mod board;
mod heuristic;
mod tree;

use crate::board::{Board, Player, Field};
use crate::tree::Solver;
use clap::Parser;
use std::io::{self, BufRead};
use std::time::Instant;
use std::sync::atomic::AtomicU64;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'm', long, help = "Liczba wierszy")]
    m: usize,

    #[arg(short = 'n', long, help = "Liczba kolumn")]
    n: usize,

    #[arg(short = 'd', long, help = "Głębokość przeszukiwania")]
    depth: usize,

    #[arg(short = 'w', long, help = "Heurystyka Białego (1-3)", default_value_t = 1)]
    h_white: usize,

    #[arg(short = 'b', long, help = "Heurystyka Czarnego (1-3)", default_value_t = 1)]
    h_black: usize,
}

fn main() {
    let args = Args::parse();

    let h_white = heuristic::get_heuristic(args.h_white);
    let h_black = heuristic::get_heuristic(args.h_black);

    let stdin = io::stdin();
    let mut fields = Vec::new();

    eprintln!("Wczytywanie planszy {}x{} ze stdin...", args.m, args.n);


    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.trim().is_empty() { continue; }

        let row: Vec<Field> = l.split_whitespace().map(|s| match s {
            "W" => Field::OCCUPIED(Player::WHITE),
            "B" => Field::OCCUPIED(Player::BLACK),
            "o" | "_" => Field::EMPTY,
            _ => Field::EMPTY,
        }).collect();

        if row.len() >= args.n {
            fields.push(row[..args.n].to_vec());
        }

        if fields.len() == args.m { break; }
    }

    if fields.len() != args.m {
        eprintln!("Błąd: Wczytano {} wierszy, a oczekiwano {}. Sprawdź wymiary -m i -n.", fields.len(), args.m);
        return;
    }

    let mut board = Board::new(args.m, args.n, fields);
    let mut current_player = Player::WHITE;
    let s = Instant::now();

    println!("Start gry Breakthrough (Plansza {}x{}, Głębokość {})", args.m, args.n, args.depth);

    for r in 1..500 {
        let nodes = AtomicU64::new(0);
        let start = Instant::now();


        let h = if current_player == Player::WHITE { &*h_white } else { &*h_black };


        let next_move = Solver::get_best_move(&board, args.depth, current_player, h, &nodes);

        let duration = start.elapsed();
        eprintln!("Runda {}: {:?} | Węzły: {} | Czas: {:?}", r, current_player, nodes.load(std::sync::atomic::Ordering::Relaxed), duration);

        if let Some(nb) = next_move {
            board = nb;
        } else {
            println!("Brak możliwych ruchów dla {:?}", current_player);
            break;
        }


        println!("{}", board);

        if check_win(&board) {
            println!("--- WYNIK ---");
            println!("Liczba rund: {}", r);
            println!("Zwycięzca: {:?}", current_player);
            println!("Czas: {:?}", s.elapsed());
            break;
        }
        current_player = current_player.opponent();
    }
}

fn check_win(board: &Board) -> bool {
    for j in 0..board.n {
        if board.fields[board.m - 1][j] == Field::OCCUPIED(Player::WHITE) { return true; }
        if board.fields[0][j] == Field::OCCUPIED(Player::BLACK) { return true; }
    }
    false
}