use crate::data::board::Board;
use crate::data::Plot;
use crate::pathfind::Frontier;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

pub fn search(
    board: Board,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    if board[start] == Plot::Obstacle || board[end] == Plot::Obstacle {
        panic!("ERROR: Start and/or end plot is an obstacle!");
    }

    let mut frontier: Frontier = Frontier::new();
    let mut g_costs: HashMap<(usize, usize), u32> = HashMap::new();
    let mut parent_to: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    frontier.enqueue(start, 0.0);
    g_costs.insert(start, 0);

    while let Some(current_plot) = frontier.get_next() {
        if current_plot == end {
            return Some(get_path(parent_to, end));
        }

        let current_g_cost = g_costs.get(&current_plot).unwrap().clone();
        for (neighbour_plot, neighbour_cost) in board.get_neighbours(current_plot) {
            let new_g = current_g_cost + neighbour_cost;

            match g_costs.entry(neighbour_plot) {
                Occupied(mut plot) => {
                    if *plot.get() <= new_g {
                        continue;
                    }
                    plot.insert(new_g);
                }
                Vacant(plot) => {
                    plot.insert(new_g);
                }
            }

            let f = new_g as f32 + heuristic(neighbour_plot, end);

            frontier.enqueue(neighbour_plot, f);

            parent_to.insert(neighbour_plot, current_plot);
        }
    }
    None
}

fn heuristic((start_x, start_y): (usize, usize), (end_x, end_y): (usize, usize)) -> f32 {
    //Euclidian distance
    let x_diff = start_x as isize - end_x as isize;
    let y_diff = start_y as isize - end_y as isize;
    ((x_diff.pow(2) + y_diff.pow(2)) as f32).sqrt()
}

fn get_path(
    parent_map: HashMap<(usize, usize), (usize, usize)>,
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut current = end;
    path.push(current);
    while let Some(&parent) = parent_map.get(&current) {
        path.push(parent);
        current = parent;
    }
    path.reverse();

    path
}
