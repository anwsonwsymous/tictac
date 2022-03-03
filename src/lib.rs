use std::fmt;
use std::io::stdin;

fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Fuck off... you need to input index (number)");
    buffer
}

fn translate_index(index: usize) -> Result<(usize, usize), &'static str> {
    match index {
        1 => Ok((0, 0)),
        2 => Ok((0, 1)),
        3 => Ok((0, 2)),
        4 => Ok((1, 0)),
        5 => Ok((1, 1)),
        6 => Ok((1, 2)),
        7 => Ok((2, 0)),
        8 => Ok((2, 1)),
        9 => Ok((2, 2)),
        _ => Err("Index not found. allowed index from 1 to 9")
    }
}

enum GameResult {
    Win(Player),
    Draw,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            GameResult::Draw => "Draw!!!\n".to_string(),
            GameResult::Win(player) => format!("{} Wins!!!\n", player)
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Player::X => "X",
            Player::O => "O"
        })
    }
}

impl Into<char> for Player {
    fn into(self) -> char {
        self.to_string().chars().next().unwrap()
    }
}

pub struct Game {
    board: Board,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            player: Player::X,
        }
    }

    pub fn start(&mut self) {
        let result = loop {
            println!("It's player {} turn", self.player);
            println!("{}", self.board);

            let index = get_input().trim().parse::<usize>().unwrap_or_default();

            match self.board.select_by_index(index, self.player.into()) {
                Ok(_) => {
                    if self.board.is_full_filled() {
                        break GameResult::Draw;
                    } else if self.board.check_winner(self.player.into()) {
                        break GameResult::Win(self.player);
                    } else {
                        self.player = if self.player == Player::X { Player::O } else { Player::X };
                    }
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        };

        println!("{}", self.board);
        println!("{}", result);
        println!("Starting Again\n");
        self.restart();
    }

    fn restart(&mut self) {
        self.board = Board::new();
        self.start();
    }
}

const EMPTY_SLOT: char = '-';

struct Board {
    matrix: Vec<Vec<char>>
}

impl Board {
    fn new() -> Self {
        Board {
            matrix: vec![vec![EMPTY_SLOT; 3]; 3]
        }
    }

    fn select_by_index(&mut self, index: usize, ch: char) -> Result<(), &str> {
        match translate_index(index) {
            Ok((y, x)) => self.select_by_coordinate(y, x, ch),
            _ => Err("Index out of bounds")
        }
    }

    fn select_by_coordinate(&mut self, y: usize, x: usize, ch: char) -> Result<(), &str> {
        if !self.is_coordinate_available(y, x) {
            return Err("This position is checked or out of bounds. Select another one.");
        }

        self.matrix[y][x] = ch;
        Ok(())
    }

    fn is_coordinate_available(&self, y: usize, x: usize) -> bool {
        match self.matrix.get(y) {
            Some(vec) => {
                match vec.get(x) {
                    Some(res) => {
                        *res == EMPTY_SLOT
                    }
                    _ => false
                }
            }
            _ => false
        }
    }

    fn is_full_filled(&self) -> bool {
        for row in &self.matrix {
            if row.iter().any(|&x| x == EMPTY_SLOT) {
                return false;
            }
        }

        true
    }

    fn check_winner(&self, ch: char) -> bool {
        let winner_vec = vec![ch, ch, ch];

        self.check_horizontal(&winner_vec) ||
        self.check_vertical(&winner_vec) ||
        self.check_diagonal(&winner_vec)
    }

    fn check_horizontal(&self, winner_vec: &Vec<char>) -> bool {
        self.matrix.iter().any(|row| row == winner_vec)
    }

    fn check_vertical(&self, winner_vec: &Vec<char>) -> bool {
        for x in 0..3 {
            if vec![self.matrix[0][x], self.matrix[1][x], self.matrix[2][x]] == *winner_vec {
                return true;
            }
        }
        false
    }

    fn check_diagonal(&self, winner_vec: &Vec<char>) -> bool {
        vec![self.matrix[0][0], self.matrix[1][1], self.matrix[2][2]] == *winner_vec ||
        vec![self.matrix[0][2], self.matrix[1][1], self.matrix[2][0]] == *winner_vec
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
           "|{}|{}|{}|\n\
            |{}|{}|{}|\n\
            |{}|{}|{}|\n",
            self.matrix[0][0], self.matrix[0][1], self.matrix[0][2],
            self.matrix[1][0], self.matrix[1][1], self.matrix[1][2],
            self.matrix[2][0], self.matrix[2][1], self.matrix[2][2],
        )
    }
}
