use crate::data::Plot;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::ops::Index;
use std::ops::IndexMut;

use std::fmt;

pub struct Board {
    pub width: usize,
    pub height: usize,
    plots: Vec<Plot>,
}

impl Index<(usize, usize)> for Board {
    type Output = Plot;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.plots[index.0 * self.height + index.1]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.plots[index.0 * self.height + index.1]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for width in 0..self.width {
            for height in 0..self.height {
                write!(f, "{} ", self[(width, height)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            width: 0,
            height: 0,
            plots: Vec::new(),
        }
    }

    // TODO: fix loading from file multiple times
    pub fn load_from_file(&mut self, filepath: &str) {
        self.clear();

        let file: File = match File::open(filepath) {
            Err(error) => panic!(
                "ERROR: Failed to open file \"{}\". Details: {}",
                filepath, error
            ),
            Ok(file) => file,
        };

        let lines = BufReader::new(file).lines();

        let mut height: usize = 0;
        let mut width: Option<usize> = None;

        for line_data in lines {
            height += 1;

            let line: String = line_data.expect("ERROR: Invalid line data!");
            let raw_plots = line.split(',');

            let mut row_width = 0;
            for raw_plot in raw_plots {
                row_width += 1;

                self.plots.push(match raw_plot.trim().parse() {
                    Ok(num) => Plot::Movable(num),
                    Err(_) => {
                        match raw_plot.trim() {
                            "S" => Plot::Start,
                            "E" => Plot::End,
                            "X" => Plot::Obstacle, //lmao
                            _ => Plot::Movable(0),
                        }
                    }
                });
            }

            match width {
                Some(global_width) => {
                    if global_width != row_width {
                        panic!("ERROR: Line width mismatch!")
                    }
                }
                None => width = Some(row_width),
            }
        }
        self.width = width.unwrap_or(0);
        self.height = height;
    }

    pub fn clear(&mut self) {
        self.width = 0;
        self.height = 0;
        self.plots.clear();
    }
}
