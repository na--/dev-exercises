use std::fmt::{self, Display, Write};
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
pub enum GameError {
    ParseError(String),
    BadGuess(String),
    InvalidSolution(String),
    GameOver,
}

impl Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO")
    }
}

#[derive(Debug)]
pub enum Command {
    TryLetter(char),
    TryWord(String),
    Info,
    Help,
    Quit,
}

impl FromStr for Command {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(GameError::ParseError("TODO".to_string()))
    }
}


pub struct Game {
    pub attempted_letters: HashSet<char>,
    pub attempted_words: HashSet<String>,
    pub attempts_remaining: u32,

    // TODO
}

impl Game {
    pub fn new(solution: &str, attempts: u32) -> Result<Self, GameError> {
        Err(GameError::InvalidSolution("TODO".to_string()))
    }

    pub fn guess_letter(&mut self, guess: char) -> Result<bool, GameError> {
        Err(GameError::BadGuess("TODO".to_string()))
    }

    pub fn guess_word(&mut self, guess: &str) -> Result<bool, GameError> {
        Err(GameError::BadGuess("TODO".to_string()))
    }

    pub fn is_over(&self) -> bool {
        false //TODO
    }
}


impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO")
    }
}
