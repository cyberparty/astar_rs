use std::fmt;

pub mod board;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Plot {
    Movable(u32),
    Obstacle,
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Plot::Movable(num) => write!(f, "{}", num),
            Plot::Obstacle => write!(f, "X"),
        }
    }
}
