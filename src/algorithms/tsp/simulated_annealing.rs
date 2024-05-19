/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use rand::{thread_rng, Rng};
use std::f64;

use crate::models::path::Path;

pub struct SimulatedAnnealingTSP;

/**
 * Simulated annealing algorithm for the TSP problem.
 */
impl SimulatedAnnealingTSP {
    pub fn solve(path: &Path) -> Path {
        let mut rng = thread_rng();
        let mut current_solution = path.clone();
        let mut temperature = 1000.0;
        let cooling_rate = 0.0003;
        let mut best_solution = current_solution.clone();

        while temperature > 1.0 {
            let new_solution = Self::get_neighbor(&current_solution);
            let current_energy = current_solution.length();
            let new_energy = new_solution.length();
            let energy_delta = new_energy - current_energy;

            if energy_delta < 0.0 || rng.gen::<f64>() < (-energy_delta / temperature).exp() {
                current_solution = new_solution;
                if current_energy < best_solution.length() {
                    best_solution = current_solution.clone();
                }
            }

            temperature *= 1.0 - cooling_rate;
        }

        best_solution
    }

    fn get_neighbor(path: &Path) -> Path {
        let mut rng = thread_rng();
        let mut new_path = path.clone();
        let index1 = rng.gen_range(0..new_path.nodes.len());
        let index2 = rng.gen_range(0..new_path.nodes.len());

        new_path.nodes.swap(index1, index2);
        new_path
    }
}
