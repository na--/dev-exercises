use std::fmt::{self, Display};
use std::str::FromStr;
use std::collections::HashSet;
use std::iter::FromIterator;

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
                (Some(ref t), Some(ref l))
                    if ("letter".starts_with(t) && l.chars().count() == 1) =>
                {
                    Ok(Command::TryLetter(l.chars().next().unwrap()))
                }
                (Some(ref t), Some(ref w)) if "word".starts_with(t) => {
                    Ok(Command::TryWord(w.to_string()))
                }
                _ => Err(GameError::ParseError("invalid try combo".to_string())),
            },
            Some(ref s) => Err(GameError::ParseError(format!(
                "the invalid command '{}'",
                s
            ))),
            None => Err(GameError::ParseError("an empty string".to_string())),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub attempted_letters: HashSet<char>,
    pub attempted_words: HashSet<String>,
    pub attempts_remaining: u32,
    actual_word: String,
    remaining_letters: HashSet<char>,
}

impl Game {
    pub fn new(solution: &str, attempts: u32) -> Result<Self, GameError> {
        if solution.len() == 0 || !solution.chars().all(char::is_alphabetic) {
            return Err(GameError::InvalidSolution(solution.to_string()));
        }
        Ok(Game {
            attempted_letters: HashSet::new(),
            attempted_words: HashSet::new(),
            attempts_remaining: attempts,
            actual_word: solution.to_string(),
            remaining_letters: HashSet::from_iter(solution.chars()),
        })
    }

    pub fn guess_letter(&mut self, guess: char) -> Result<bool, GameError> {
        if self.is_over() {
            return Err(GameError::GameOver);
        }
        if self.attempted_letters.contains(&guess) {
            return Err(GameError::BadGuess(guess.to_string()));
        }

        self.attempted_letters.insert(guess);
        self.remaining_letters.remove(&guess);

        if self.actual_word.contains(guess) {
            return Ok(true);
        }

        self.attempts_remaining -= 1;
        return Ok(false);
    }

    pub fn guess_word(&mut self, guess: &str) -> Result<bool, GameError> {
        if self.is_over() {
            return Err(GameError::GameOver);
        }
        if self.attempted_words.contains(guess) {
            return Err(GameError::BadGuess(guess.to_string()));
        }

        self.attempted_words.insert(guess.to_string());

        if self.actual_word == guess {
            return Ok(true);
        }

        self.attempts_remaining -= 1;
        return Ok(false);
    }

    pub fn is_over(&self) -> bool {
        return self.attempts_remaining == 0 || self.attempted_words.contains(&self.actual_word)
            || self.remaining_letters.is_empty();
    }
}


impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.attempts_remaining == 0 {
            write!(f, "You lost! :/\nThe word was: {}", self.actual_word)
        } else if self.attempted_words.contains(&self.actual_word)
            || self.remaining_letters.is_empty()
        {
            write!(f, "You won! ^_^\nThe word was: {}", self.actual_word)
        } else {
            let current_guess: String = self.actual_word
                .chars()
                .map(|ch| {
                    if self.attempted_letters.contains(&ch) {
                        format!(" {}", ch)
                    } else {
                        String::from(" _")
                    }
                })
                .collect();

            write!(
                f,
                "Attempts remaining: {}\nGuess:{}",
                self.attempts_remaining, current_guess
            )
        }
    }
}
