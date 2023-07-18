use std::{
    cmp::Ordering,
    collections::hash_map::Entry::{Occupied, Vacant},
    collections::{BTreeSet, HashMap},
};

pub mod astar;

#[derive(Debug)]
struct PlotPriority {
    cost: f32,
    coords: (usize, usize),
}

impl Ord for PlotPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost != other.cost {
            return self.cost.total_cmp(&other.cost);
        }
        self.coords.cmp(&other.coords)
    }
}

impl PartialOrd for PlotPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PlotPriority {}

impl PartialEq for PlotPriority {
    fn eq(&self, other: &Self) -> bool {
        (self.cost, self.coords) == (other.cost, other.coords)
    }
}

pub struct Frontier {
    queue: BTreeSet<PlotPriority>,
    visited: HashMap<(usize, usize), f32>,
}

impl Frontier {
    fn new() -> Self {
        Self {
            queue: BTreeSet::new(),
            visited: HashMap::new(),
        }
    }

    fn enqueue(&mut self, plot_coords: (usize, usize), plot_cost: f32) {
        self.queue.insert(PlotPriority {
            coords: plot_coords,
            cost: plot_cost,
        });
    }

    fn get_next(&mut self) -> Option<(usize, usize)> {
        while let Some(PlotPriority {
            coords: lowest_coords,
            cost: lowest_cost,
        }) = self.queue.pop_first()
        {
            match self.visited.entry(lowest_coords) {
                Occupied(mut plot) => {
                    if *plot.get() <= lowest_cost {
                        continue;
                    }
                    plot.insert(lowest_cost);
                }
                Vacant(plot) => {
                    plot.insert(lowest_cost);
                }
            };
            return Some(lowest_coords);
        }
        None
    }
}
