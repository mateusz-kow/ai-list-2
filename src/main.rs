mod board;
mod heuristic;
mod tree;

use crate::board::{Board, Player, Field};
use crate::tree::Solver;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::sync::atomic::AtomicU64;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'm', long, help = "Liczba wierszy", default_value_t = 8)]
    m: usize,

    #[arg(short = 'n', long, help = "Liczba kolumn", default_value_t = 8)]
    n: usize,

    #[arg(short = 'd', long, help = "Głębokość przeszukiwania", default_value_t = 5)]
    depth: usize,

    #[arg(short = 'w', long, help = "Heurystyka Białego (1-3)", default_value_t = 3)]
    h_white: usize,

    #[arg(short = 'b', long, help = "Heurystyka Czarnego (1-3)", default_value_t = 1)]
    h_black: usize,

    #[arg(short = 'f', long, help = "Opcjonalna ścieżka do pliku z planszą (.txt)")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let h_white = heuristic::get_heuristic(args.h_white);
    let h_black = heuristic::get_heuristic(args.h_black);


    let board = if let Some(path) = args.file {
        eprintln!("Wczytywanie planszy z pliku: {:?}", path);
        let file = File::open(path).expect("Nie można otworzyć pliku");
        let reader = BufReader::new(file);
        let mut fields = Vec::new();

        for line in reader.lines() {
            let l = line.unwrap();
            if l.trim().is_empty() { continue; }
            let row: Vec<Field> = l.split_whitespace().map(|s| match s {
                "W" => Field::OCCUPIED(Player::WHITE),
                "B" => Field::OCCUPIED(Player::BLACK),
                _ => Field::EMPTY,
            }).collect();

            if row.len() >= args.n {
                fields.push(row[..args.n].to_vec());
            }
            if fields.len() == args.m { break; }
        }
        Board::new(args.m, args.n, fields)
    } else {
        eprintln!("Brak pliku wejściowego. Generowanie domyślnej planszy {}x{}", args.m, args.n);
        Board::default_start(args.m, args.n)
    };

    let mut current_board = board;
    let mut current_player = Player::WHITE;
    let s = Instant::now();

    println!("Start gry Breakthrough (Głębokość: {})", args.depth);
    println!("Stan początkowy:\n{}", current_board);

    for r in 1..500 {
        let nodes = AtomicU64::new(0);
        let start = Instant::now();

        let h = if current_player == Player::WHITE { &*h_white } else { &*h_black };
        let next_move = Solver::get_best_move(&current_board, args.depth, current_player, h, &nodes);

        let duration = start.elapsed();
        eprintln!("Runda {}: {:?} | Węzły: {} | Czas: {:?}", r, current_player, nodes.load(std::sync::atomic::Ordering::Relaxed), duration);

        if let Some(nb) = next_move {
            current_board = nb;
        } else {
            println!("Koniec ruchów dla {:?}", current_player);
            break;
        }

        println!("{}", current_board);

        if check_win(&current_board) {
            println!("--- KONIEC ROZGRYWKI ---");
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