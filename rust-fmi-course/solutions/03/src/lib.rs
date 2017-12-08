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

static GAME_OVER_MAN: &'static str = r#"
"That's it, man. Game over, man. Game over! What the fuck are we gonna do now? What are we gonna do?"
"Maybe we can build a fire, sing a couple of songs, huh? Why don't we try that?"
"We'd better get back 'cause it'll be dark soon and they mostly come at night. Mostly."
"#;

impl Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GameError::ParseError(ref s) => write!(f, "Error! Could not parse {}", s),
            &GameError::BadGuess(ref guess) => {
                write!(f, "Error! You already tried to guess '{}'", guess)
            }
            &GameError::InvalidSolution(ref s) => {
                write!(f, "Error! Invalid solution argument '{}'!", s)
            }
            &GameError::GameOver => write!(f, "{}", GAME_OVER_MAN),
        }
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
        let s = s.trim().to_lowercase();
        let mut words = s.split_whitespace();
        match words.next() {
            Some(ref s) if "help".starts_with(s) => Ok(Command::Help),
            Some(ref s) if "info".starts_with(s) => Ok(Command::Info),
            Some(ref s) if "quit".starts_with(s) => Ok(Command::Quit),
            Some(ref s) if "try".starts_with(s) => match (words.next(), words.next()) {
                (Some(ref t), Some(ref l)) if ("letter".starts_with(t) && l.len() == 1) => Ok(Command::TryLetter(l.chars().next().unwrap())),
                (Some(ref t), Some(ref w)) if "word".starts_with(t) => Ok(Command::TryWord(w.to_string())),
                _ => Err(GameError::ParseError("invalid try combo".to_string()))
            },
            Some(ref s) => Err(GameError::ParseError(format!("the invalid command '{}'", s))),
            None => Err(GameError::ParseError("an empty string".to_string()))
        }
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
