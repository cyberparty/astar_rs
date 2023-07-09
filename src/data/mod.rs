use std::fmt;

pub mod board;

pub enum Plot {
    Movable(i32),
    Obstacle,
    Start,
    End,
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Plot::Movable(num) => write!(f, "{}", num),
            Plot::Obstacle => write!(f, "X"),
            Plot::Start => write!(f, "S"),
            Plot::End => write!(f, "E"),
        }
    }
}
