use crate::board::{Board, Player};
use crate::heuristic::Heuristic;
use dashmap::DashMap;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, PartialEq)]
enum NodeTag { Exact, Alpha, Beta }

struct CacheEntry {
    val: i32,
    depth: usize,
    tag: NodeTag,
}

type Cache = DashMap<u64, CacheEntry>;

pub struct Solver;

impl Solver {
    pub fn get_best_move(board: &Board, depth: usize, player: Player, heuristic: &dyn Heuristic, nodes: &AtomicU64) -> Option<Board> {
        let mut moves = board.get_possible_moves(player);
        if moves.is_empty() { return None; }

        let is_white = player == Player::WHITE;
        let cache = DashMap::new();

        moves.sort_by_cached_key(|m| {
            let score = heuristic.eval_state(m);
            if is_white { -score } else { score }
        });

        moves.into_par_iter().map(|m| {
            let val = Self::alpha_beta(&m, depth - 1, -32000, 32000, !is_white, player.opponent(), heuristic, nodes, &cache);
            (m, val)
        }).max_by_key(|x| if is_white { x.1 } else { -x.1 })
            .map(|x| x.0)
    }

    fn alpha_beta(
        board: &Board,
        depth: usize,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
        curr_player: Player,
        heuristic: &dyn Heuristic,
        nodes: &AtomicU64,
        cache: &Cache
    ) -> i32 {
        nodes.fetch_add(1, Ordering::Relaxed);

        if let Some(entry) = cache.get(&board.hash) {
            if entry.depth >= depth {
                match entry.tag {
                    NodeTag::Exact => return entry.val,
                    NodeTag::Alpha => alpha = alpha.max(entry.val),
                    NodeTag::Beta => beta = beta.min(entry.val),
                }
                if alpha >= beta { return entry.val; }
            }
        }

        if depth == 0 || Self::is_terminal(board) {
            return heuristic.eval_state(board);
        }

        let mut moves = board.get_possible_moves(curr_player);
        if moves.is_empty() { return heuristic.eval_state(board); }

        moves.sort_by_cached_key(|m| {
            let score = heuristic.eval_state(m);
            if maximizing { -score } else { score }
        });

        let mut best_val = if maximizing { -32000 } else { 32000 };
        let original_alpha = alpha;

        for m in moves {
            let val = Self::alpha_beta(&m, depth - 1, alpha, beta, !maximizing, curr_player.opponent(), heuristic, nodes, cache);
            if maximizing {
                best_val = best_val.max(val);
                alpha = alpha.max(best_val);
            } else {
                best_val = best_val.min(val);
                beta = beta.min(best_val);
            }
            if beta <= alpha { break; }
        }

        let tag = if best_val <= original_alpha { NodeTag::Beta }
        else if best_val >= beta { NodeTag::Alpha }
        else { NodeTag::Exact };

        cache.insert(board.hash, CacheEntry { val: best_val, depth, tag });
        best_val
    }

    fn is_terminal(board: &Board) -> bool {
        for j in 0..board.n {
            if board.fields[board.m-1][j] == crate::board::Field::OCCUPIED(Player::WHITE) { return true; }
            if board.fields[0][j] == crate::board::Field::OCCUPIED(Player::BLACK) { return true; }
        }
        false
    }
}