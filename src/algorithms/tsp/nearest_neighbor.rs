/*
 * Copyright (c) 2024 Filippo Finke
 */

use crate::models::path::Path;

pub struct NearestNeighborTSP;

impl NearestNeighborTSP {
    pub fn solve(path: &Path) -> Path {
        let mut unvisited = path.nodes.clone();
        let mut current_node = unvisited.pop().unwrap();
        let mut visited = vec![current_node];

        while !unvisited.is_empty() {
            let nearest_index =
                NearestNeighborTSP::find_nearest_neighbor(&current_node, &unvisited);
            current_node = unvisited.remove(nearest_index);
            visited.push(current_node);
        }

        visited.push(visited[0].clone());
        Path { nodes: visited }
    }

    fn find_nearest_neighbor(
        current_node: &(i32, i32, i32),
        unvisited: &Vec<(i32, i32, i32)>,
    ) -> usize {
        let mut min_index = 0;
        let mut min_distance = f64::INFINITY;
        for (index, node) in unvisited.iter().enumerate() {
            let distance = NearestNeighborTSP::distance(current_node, node);
            if distance < min_distance {
                min_distance = distance;
                min_index = index;
            }
        }
        min_index
    }

    fn distance(node1: &(i32, i32, i32), node2: &(i32, i32, i32)) -> f64 {
        let (_, x1, y1) = node1;
        let (_, x2, y2) = node2;
        let dx = x2 - x1;
        let dy = y2 - y1;
        ((dx * dx + dy * dy) as f64).sqrt()
    }
}
