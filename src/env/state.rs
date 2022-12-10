use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    X = 1,
    O = -1,
    Empty = 0,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::X => write!(f, "X"),
            State::O => write!(f, "O"),
            State::Empty => write!(f, " "),
        }
    }
}

