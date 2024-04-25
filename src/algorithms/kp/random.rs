use rand::{thread_rng, Rng};

use crate::models::{instance::Instance, path::Path};

pub struct RandomKP;

impl RandomKP {
    pub fn solve(path: &Path, instance: &Instance) -> f64 {
        let mut items: Vec<(f64, f64)> = vec![];
        for item in instance.items.iter() {
            // Check if the node is in the path
            if path.has_node(item.3 as i32) {
                items.push((item.1, item.2));
            }
        }

        let max_capacity = instance.capacity_of_knapsack;
        let mut current_weight: f64 = 0.0;
        let mut current_profit: f64 = 0.0;

        // Generate a random solution
        // Randomly select items to put in the knapsack
        let mut rng = thread_rng();
        for _ in 0..items.len() {
            // Randomly select an item
            // Get random item from vector and remove it
            let index = rng.gen_range(0..=(items.len() - 1));

            let (weight, profit) = items[index];
            items.remove(index);

            // Check if adding the item exceeds the capacity
            if current_weight + weight <= max_capacity {
                // Add the item to the solution
                current_weight += weight;
                current_profit += profit;
            }
        }

        println!(
            "Current capacity: {} Current profit: {} Max capacity: {}",
            current_weight, current_profit, max_capacity
        );

        current_profit
    }
}
