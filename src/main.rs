/*
 * Created on Tue Mar 26 2024
 *
 * Copyright (c) 2024 Filippo Finke
 */

use instance::Instance;

use crate::{
    algorithms::tsp::{
        brute_force::BruteForceTSP, nearest_insertion::NearestInsertionTSP,
        nearest_neighbor::NearestNeighborTSP, two_opt::TwoOpt,
    },
    models::path::Path,
};

mod algorithms;
mod instance;
mod models;
mod solution;

fn main() {
    // let instance = Instance::load("./instances/test.ttp").expect("Failed to load instance");
    let instance = Instance::load("./instances/a280_n279_bounded-strongly-corr_01.ttp")
        .expect("Failed to load instance");

    println!("{}\n", instance);

    /*println!("Brute force TSP");
    let path = Path::new(instance.node_coords.clone());
    println!("Initial path length: {}", path.length());
    let shortest_path = BruteForceTSP::solve(&path);
    println!("Shortest path length: {}", shortest_path.length());
    */

    println!("\nNearest neighbor TSP");
    let path = Path::new(instance.node_coords.clone());
    println!("Initial path length: {}", path.length());
    let shortest_path = NearestNeighborTSP::solve(&path);
    println!("Shortest path length: {}", shortest_path.length());

    println!("\nNearest insertion TSP");
    let path = Path::new(instance.node_coords.clone());
    println!("Initial path length: {}", path.length());
    let shortest_path = NearestInsertionTSP::solve(&path);
    println!("Shortest path length: {}", shortest_path.length());

    println!("\nTwo opt TSP");
    let path = Path::new(instance.node_coords.clone());
    println!("Initial path length: {}", path.length());
    let shortest_path = TwoOpt::solve(&path);
    println!("Shortest path length: {}", shortest_path.length());
}
