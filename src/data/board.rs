use crate::data::Plot;
use std::collections::HashMap;
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

        let mut y_index: usize = 0;

        for line_data in lines {
            let line: String = line_data.expect("ERROR: Invalid line data!");
            let raw_plots = line.split(',');

            let mut x_index = 0;
            for raw_plot in raw_plots {
                self.plots.push(match raw_plot.trim().parse() {
                    Ok(num) => Plot::Movable(num),
                    Err(_) => match raw_plot.trim() {
                        "X" => Plot::Obstacle,
                        _ => Plot::Movable(0),
                    },
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
    }

    pub fn get_neighbours(&self, plot: (usize, usize)) -> HashMap<(usize, usize), i32> {
        let (plot_x, plot_y) = plot;
        let plot_deltas: Vec<(isize, isize)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let mut neighbours: HashMap<(usize, usize), i32> = HashMap::new();

        for (delta_x, delta_y) in plot_deltas {
            if let (Some(new_x), Some(new_y)) = (
                plot_x.checked_add_signed(delta_x),
                plot_y.checked_add_signed(delta_y),
            ) {
                if new_x < self.width || new_y < self.height {
                    match self[(new_x, new_y)] {
                        Plot::Movable(num) => neighbours.insert((new_x, new_y), num),
                        Plot::Obstacle => continue,
                    };
                }
            }
        }
        neighbours
    }

    pub fn clear(&mut self) {
        self.width = 0;
        self.height = 0;
        self.plots.clear();
    }
}
