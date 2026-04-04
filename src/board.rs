use std::error::Error;
use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum Player {
    #[serde(rename = "W")]
    WHITE,
    #[serde(rename = "B")]
    BLACK,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::WHITE => Player::BLACK,
            Player::BLACK => Player::WHITE,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[serde(untagged)]
pub enum Field {
    OCCUPIED(Player),
    #[serde(rename = "_")]
    EMPTY,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub fields: Vec<Vec<Field>>,
}

impl Board {
    pub fn new(m: usize, n: usize) -> Board {
        Board {
            fields: Board::init_board(m, n).unwrap(),
        }
    }

    pub fn get_possible_moves(&self, player: &Player) -> Vec<Board> {
        let mut result: Vec<Board> = Vec::new();

        for (i, row) in self.fields.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                let is_player = match field {
                    Field::OCCUPIED(p) if p == player => true,
                    _ => false,
                };

                if is_player {
                    result.append(&mut self.get_possible_player_moves(player, i, j));
                }
            }
        }

        result
    }

    fn get_possible_player_moves(&self, player: &Player, i: usize, j: usize) -> Vec<Board> {
        let mut result: Vec<Board> = Vec::new();

        let height = self.fields.len();
        let width = self.fields[0].len();

        match player {
            Player::WHITE => {
                let new_j = j + 1;
                if new_j < width {
                    if self.fields[i][new_j] == Field::EMPTY {
                        result.push(self.move_player(i, j, i, new_j));
                    }

                    if i + 1 < height {
                        result.push(self.move_player(i, j, i + 1, new_j));
                    }

                    if i > 0 {
                        result.push(self.move_player(i, j, i - 1, new_j));
                    }
                }
            }

            Player::BLACK => {
                if j > 0 {
                    let new_j = j - 1;

                    if self.fields[i][new_j] == Field::EMPTY {
                        result.push(self.move_player(i, j, i, new_j));
                    }

                    if i + 1 < height {
                        result.push(self.move_player(i, j, i + 1, new_j));
                    }

                    if i > 0 {
                        result.push(self.move_player(i, j, i - 1, new_j));
                    }
                }
            }
        }

        result
    }

    fn move_player(&self, i: usize, j: usize, i_to: usize, j_to: usize) -> Board {
        let mut new_fields: Vec<Vec<Field>> = self.fields.clone();
        new_fields[i_to][j_to] = new_fields[i][j];
        new_fields[i][j] = Field::EMPTY;

        Board {fields: new_fields}
    }
    fn init_board(m: usize, n: usize) -> Result<Vec<Vec<Field>>, Box<dyn Error>> {
        let mut board = vec![vec![Field::EMPTY; n]; m];

        if m == 0 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "'m' must be greater than 0",
            )));
        }
        if n < 4 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "'n' must be greater than 4",
            )));
        }

        for i in 0..m {
            for j in 0..2 {
                board[i][j] = Field::OCCUPIED(Player::WHITE);
            }

            for j in n - 2..n {
                board[i][j] = Field::OCCUPIED(Player::BLACK);
            }
        }

        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}