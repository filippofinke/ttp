/*
 * Created on Tue Mar 26 2024
 *
 * Copyright (c) 2024 Filippo Finke
 */

use instance::Instance;

use crate::algorithms::{algorithm::Algorithm, simple_heuristic::SimpleHeuristic};

mod algorithms;
mod instance;
mod solution;

fn main() {
    let instance = Instance::load("./instances/a280_n279_bounded-strongly-corr_01.ttp")
        .expect("Failed to load instance");

    println!("{}\n", instance);
    println!("Solutions: ");
    let solutions = SimpleHeuristic.solve(&instance);
    for solution in solutions {
        print!("{}", solution);
    }
}
