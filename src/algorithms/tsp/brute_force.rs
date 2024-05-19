/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use crate::models::path::Path;

pub struct BruteForceTSP;

// Helper function to generate all the possible permutations of the nodes.
fn permute(nodes: &mut Vec<(i32, i32, i32)>, start: usize, result: &mut Vec<Vec<(i32, i32, i32)>>) {
    if start == nodes.len() {
        result.push(nodes.clone());
    } else {
        for i in start..nodes.len() {
            nodes.swap(start, i);
            permute(nodes, start + 1, result);
            nodes.swap(start, i);
        }
    }
}

/**
 * Brute force algorithm for the TSP problem.
 * Try all the possible permutations of the nodes and return the shortest path.
 */
impl BruteForceTSP {
    pub fn solve(path: &Path) -> Path {
        let mut path = path.clone();
        let mut best_path = path.clone();
        let mut best_length = path.length();

        let mut all_permutations: Vec<Vec<(i32, i32, i32)>> = Vec::new();
        permute(&mut path.nodes, 0, &mut all_permutations);

        for perm in all_permutations {
            let current_path = Path { nodes: perm };
            let current_length = current_path.length();
            if current_length < best_length {
                best_length = current_length;
                best_path = current_path;
            }
        }

        best_path
    }
}
