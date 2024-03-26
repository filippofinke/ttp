/*
 * Created on Tue Mar 26 2024
 *
 * Copyright (c) 2024 Filippo Finke
 */

use instance::Instance;

mod instance;
mod solution;

fn solve_traveling_thief(instance: &Instance) {
    println!("Problem name: {:?}", instance);
}

fn main() {
    let instance = Instance::load("./instances/a280_n279_bounded-strongly-corr_01.ttp")
        .expect("Failed to load instance");

    solve_traveling_thief(&instance);
}
