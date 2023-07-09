use crate::data::Plot;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::ops::Index;
use std::ops::IndexMut;

use std::fmt;

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub start: Option<(usize, usize)>,
    pub end: Option<(usize, usize)>,
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
        writeln!(f, "Width: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            width: 0,
            height: 0,
            start: None,
            end: None,
            plots: Vec::new(),
        }
    }

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

        let mut width: Option<usize> = None;
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;

        let mut y_index: usize = 0;

        for line_data in lines {
            let line: String = line_data.expect("ERROR: Invalid line data!");
            let raw_plots = line.split(',');

            let mut x_index = 0;
            for raw_plot in raw_plots {
                self.plots.push(match raw_plot.trim().parse() {
                    Ok(num) => Plot::Movable(num),
                    Err(_) => {
                        match raw_plot.trim() {
                            "S" => {
                                if start.is_some() {
                                    panic!("ERROR: Multiple start points defined in grid!")
                                }
                                start = Some((x_index, y_index));
                                Plot::Start
                            }
                            "E" => {
                                if end.is_some() {
                                    panic!("ERROR: Multiple end points defined in grid!")
                                }
                                end = Some((x_index, y_index));
                                Plot::End
                            }
                            "X" => Plot::Obstacle, //lmao
                            _ => Plot::Movable(0),
                        }
                    }
                });
                x_index += 1;
            }

            match width {
                Some(global_width) => {
                    if global_width != x_index {
                        panic!("ERROR: Line width mismatch!")
                    }
                }
                None => width = Some(x_index),
            }

            y_index += 1;
        }

        self.width = width.unwrap_or(0);
        self.height = y_index;

        self.start = start;
        self.end = end;
    }

    pub fn clear(&mut self) {
        self.width = 0;
        self.height = 0;
        self.start = None;
        self.end = None;
        self.plots.clear();
    }
}
