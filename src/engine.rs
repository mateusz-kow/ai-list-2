use crate::board::Board;

struct GameEngine {
    board: Board,
}

impl GameEngine {
    pub fn new(board: Board) -> GameEngine {
        GameEngine { board }
    }
}