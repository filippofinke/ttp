/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use std::collections::HashSet;

use crate::models::path::Path;

pub struct TabuSearchTSB;

/**
 * Tabu search algorithm for the TSP problem.
 * Generate neighbors of the current path and select the best one.
 * Keep track of the best solution found so far and avoid visiting the same path twice.
 */

impl TabuSearchTSB {
    pub fn solve(path: &Path) -> Path {
        let mut best_solution = path.clone();
        let mut current_solution = path.clone();
        let mut tabu_list: HashSet<Path> = HashSet::new();

        let max_iterations = 100;
        let tabu_tenure = 30;
        let mut iterations = 0;

        while iterations < max_iterations {
            let neighbors = TabuSearchTSB::generate_neighbors(&current_solution);
            let mut best_neighbor = None;
            let mut best_neighbor_length = f64::INFINITY;

            for neighbor in neighbors {
                let neighbor_length = neighbor.length();
                if neighbor_length < best_neighbor_length && !tabu_list.contains(&neighbor) {
                    best_neighbor = Some(neighbor.clone());
                    best_neighbor_length = neighbor_length;
                }
            }

            if let Some(neighbor) = best_neighbor {
                current_solution = neighbor.clone();
                if neighbor.length() < best_solution.length() {
                    best_solution = neighbor.clone();
                }
                tabu_list.insert(neighbor);
                if tabu_list.len() > tabu_tenure {
                    let oldest_tabu = tabu_list.iter().next().unwrap().clone();
                    tabu_list.remove(&oldest_tabu);
                }
            }

            iterations += 1;
        }

        best_solution
    }

    fn generate_neighbors(path: &Path) -> Vec<Path> {
        let mut neighbors = Vec::new();
        let nodes_count = path.nodes.len();
        for i in 1..nodes_count {
            for j in (i + 1)..nodes_count {
                let mut new_nodes = path.nodes.clone();
                new_nodes.swap(i, j);
                neighbors.push(Path::new(new_nodes));
            }
        }
        neighbors
    }
}
