/*
 * Created on Tue Mar 26 2024
 *
 * Copyright (c) 2024 Filippo Finke
 */
use crate::{
    algorithms::tsp::{
        brute_force::BruteForceTSP, nearest_insertion::NearestInsertionTSP,
        nearest_neighbor::NearestNeighborTSP, simulated_annealing::SimulatedAnnealingTSP,
        tabu_search::TabuSearchTSB, two_opt::TwoOpt,
    },
    models::path::Path,
};
use dialoguer::{theme::ColorfulTheme, Select};
use instance::Instance;
use std::fs;

mod algorithms;
mod instance;
mod models;
mod solution;

fn main() {
    // List all the files in the instances folder
    let files = fs::read_dir("./instances")
        .expect("Failed to read instances folder")
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    Some(file_name.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    // Ask the user to select one file from the list
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an instance file")
        .default(0)
        .items(&files)
        .interact()
        .expect("Failed to select an instance file");

    // Get the selected file path
    let selected_file = &files[selection];

    println!("Selected file: {}", selected_file);

    // Load the selected instance
    let instance =
        Instance::load(&format!("./instances/{}", selected_file)).expect("Failed to load instance");

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

    println!("\nSimulated annealing TSP");
    let path = Path::new(instance.node_coords.clone());
    println!("Initial path length: {}", path.length());
    let shortest_path = SimulatedAnnealingTSP::solve(&path);
    println!("Shortest path length: {}", shortest_path.length());

    println!("\nTabu search TSP");
    let path = Path::new(instance.node_coords.clone());
    println!("Initial path length: {}", path.length());
    let shortest_path = TabuSearchTSB::solve(&path);
    println!("Shortest path length: {}", shortest_path.length());
}
