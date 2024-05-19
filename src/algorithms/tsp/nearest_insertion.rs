/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use crate::models::path::Path;

pub struct NearestInsertionTSP;

/**
 * Nearest Insertion TSP algorithm.
 * Not sure about the implementation, it was just an experiment.
 *
 * Find the nearest node to the current path and insert it in the best position.
 */
impl NearestInsertionTSP {
    pub fn solve(path: &Path) -> Path {
        let mut unvisited = path.nodes.clone();
        let mut current_node = unvisited.pop().unwrap();
        let mut visited = vec![current_node];

        while !unvisited.is_empty() {
            let (insert_index, _) =
                NearestInsertionTSP::find_nearest_insertion(&visited, &unvisited);
            current_node = unvisited.remove(insert_index);
            visited.push(current_node);
        }

        visited.push(visited[0].clone()); // Close the loop
        Path { nodes: visited }
    }

    fn find_nearest_insertion(
        visited: &Vec<(i32, i32, i32)>,
        unvisited: &Vec<(i32, i32, i32)>,
    ) -> (usize, f64) {
        let mut best_insert_index = 0;
        let mut best_insert_cost = f64::INFINITY;

        for (index, node) in unvisited.iter().enumerate() {
            for i in 0..visited.len() - 1 {
                let prev_node = &visited[i];
                let next_node = &visited[i + 1];
                let insertion_cost =
                    NearestInsertionTSP::calculate_insertion_cost(prev_node, next_node, node);
                if insertion_cost < best_insert_cost {
                    best_insert_cost = insertion_cost;
                    best_insert_index = index;
                }
            }
        }

        (best_insert_index, best_insert_cost)
    }

    fn calculate_insertion_cost(
        prev_node: &(i32, i32, i32),
        next_node: &(i32, i32, i32),
        new_node: &(i32, i32, i32),
    ) -> f64 {
        let dist_prev_to_new = NearestInsertionTSP::distance(prev_node, new_node);
        let dist_new_to_next = NearestInsertionTSP::distance(new_node, next_node);
        let dist_prev_to_next = NearestInsertionTSP::distance(prev_node, next_node);
        dist_prev_to_new + dist_new_to_next - dist_prev_to_next
    }

    fn distance(node1: &(i32, i32, i32), node2: &(i32, i32, i32)) -> f64 {
        let (_, x1, y1) = node1;
        let (_, x2, y2) = node2;
        let dx = x2 - x1;
        let dy = y2 - y1;
        ((dx * dx + dy * dy) as f64).sqrt()
    }
}
