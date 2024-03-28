/*
 * Created on Tue Mar 26 2024
 *
 * Copyright (c) 2024 Filippo Finke
 */

use instance::Instance;

use crate::{algorithms::tsp::brute_force::BruteForceTSP, models::path::Path};

mod algorithms;
mod instance;
mod models;
mod solution;

fn main() {
    let instance = Instance::load("./instances/test.ttp").expect("Failed to load instance");

    println!("{}\n", instance);

    let path = Path::new(instance.node_coords);
    println!("Initial path length: {}", path.length());
    let shortest_path = BruteForceTSP::solve(&path);
    println!("Shortest path length: {}", shortest_path.length());
}
