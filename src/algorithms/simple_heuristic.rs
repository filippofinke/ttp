use std::time::Instant;

use crate::{instance::Instance, solution::Solution};

use super::algorithm::Algorithm;

pub struct SimpleHeuristic;

impl Algorithm for SimpleHeuristic {
    fn solve(&self, problem: &Instance) -> Vec<Solution> {
        let start_time = Instant::now();
        let mut solutions = Vec::new();

        // Calculate total traveling time t0
        let mut total_traveling_time = 0.0;
        for i in 0..problem.dimension - 1 {
            let (_node_id, x1, y1) = problem.node_coords[i];
            let (_, x2, y2) = problem.node_coords[i + 1];
            let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
            total_traveling_time += distance / problem.min_speed;
        }
        total_traveling_time += ((problem.node_coords[0].1
            - problem.node_coords[problem.dimension - 1].1)
            .powi(2)
            + (problem.node_coords[0].2 - problem.node_coords[problem.dimension - 1].2).powi(2))
        .sqrt()
            / problem.min_speed;

        let mut items_with_score: Vec<(usize, f64, f64, usize, f64)> = Vec::new();

        for i in 1..problem.dimension {
            let mut city_score: Vec<(usize, f64, f64, usize, f64)> = Vec::new();

            for (item_id, profit, weight, _) in &problem.items {
                let (_node_id, x1, y1) = problem.node_coords[i];
                let (_, x2, y2) = problem.node_coords[0];
                let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                let traveling_time = distance / problem.min_speed;
                let score = profit - problem.renting_ratio * traveling_time;
                city_score.push((*item_id, *profit, *weight, i, score));
            }

            city_score.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap());

            for item in city_score {
                items_with_score.push(item);
            }
        }

        let mut packed_items: Vec<usize> = Vec::new();
        let mut total_weight = 0.0;

        for (item_id, _, weight, _, _) in items_with_score {
            if total_weight + weight <= problem.capacity_of_knapsack {
                packed_items.push(item_id);
                total_weight += weight;
            }
        }

        let mut tsp_tour = vec![0; problem.dimension];
        for i in 0..problem.dimension {
            tsp_tour[i] = i;
        }

        let solution = Solution {
            tsp_tour,
            packing_plan: packed_items,
            fp: 0.0, // Placeholder value, compute as needed
            ft: total_traveling_time,
            ftraw: 0,       // Placeholder value, compute as needed
            ob: 0.0,        // Placeholder value, compute as needed
            wend: 0.0,      // Placeholder value, compute as needed
            wend_used: 0.0, // Placeholder value, compute as needed
            computation_time: start_time.elapsed().as_secs(),
        };

        solutions.push(solution);

        solutions
    }
}
