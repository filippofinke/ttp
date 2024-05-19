/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use crate::models::path::Path;

pub struct LinKernighanTSP;

/**
 * Lin-Kernighan TSP algorithm.
 *
 * Tried to implement the Lin-Kernighan algorithm, but not sure if it's correct.
 */

impl LinKernighanTSP {
    pub fn solve(path: &Path) -> Path {
        let mut best_path = path.clone();
        let mut improved = true;

        while improved {
            improved = false;
            let n = best_path.nodes.len();
            let mut best_gain = 0.0;
            let mut best_i = 0;
            let mut best_j = 0;

            for i in 0..n {
                for j in (i + 2)..n {
                    let gain = LinKernighanTSP::calculate_gain(&best_path, i, j);
                    if gain > best_gain {
                        best_gain = gain;
                        best_i = i;
                        best_j = j;
                    }
                }
            }

            if best_gain > 0.0 {
                improved = true;
                LinKernighanTSP::perform_exchange(&mut best_path, best_i, best_j);
            }
        }

        best_path
    }

    fn calculate_gain(path: &Path, i: usize, j: usize) -> f64 {
        let n = path.nodes.len();
        let (x1, y1, _) = path.nodes[i];
        let (x2, y2, _) = path.nodes[(i + 1) % n];
        let (x3, y3, _) = path.nodes[j];
        let (x4, y4, _) = path.nodes[(j + 1) % n];

        let d1 = LinKernighanTSP::distance(x1, y1, x2, y2);
        let d2 = LinKernighanTSP::distance(x3, y3, x4, y4);
        let d3 = LinKernighanTSP::distance(x1, y1, x3, y3);
        let d4 = LinKernighanTSP::distance(x2, y2, x4, y4);

        let gain = d1 + d2 - (d3 + d4);
        gain
    }

    fn perform_exchange(path: &mut Path, i: usize, j: usize) {
        let n = path.nodes.len();
        let mut new_nodes = Vec::with_capacity(n);
        let mut idx = i;

        while idx != j {
            new_nodes.push(path.nodes[idx]);
            idx = (idx + 1) % n;
        }

        new_nodes.push(path.nodes[j]);

        idx = (j + 1) % n;

        while idx != i {
            new_nodes.push(path.nodes[idx]);
            idx = (idx + 1) % n;
        }

        new_nodes.push(path.nodes[i]);
        *path = Path::new(new_nodes);
    }

    fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let distance = ((dx * dx + dy * dy) as f64).sqrt();
        distance
    }
}
