use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Instance {
    problem_name: String,
    knapsack_data_type: String,
    dimension: usize,
    num_items: usize,
    capacity_of_knapsack: f64,
    min_speed: f64,
    max_speed: f64,
    renting_ratio: f64,
    edge_weight_type: String,
    node_coords: Vec<(usize, f64, f64)>,
    items: Vec<(usize, f64, f64, usize)>,
}

fn solve_traveling_thief(instance: &Instance) {}

fn parse_instance(file_path: &str) -> Option<Instance> {
    let file = match File::open(Path::new(file_path)) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut instance = Instance {
        problem_name: String::new(),
        knapsack_data_type: String::new(),
        dimension: 0,
        num_items: 0,
        capacity_of_knapsack: 0.0,
        min_speed: 0.0,
        max_speed: 0.0,
        renting_ratio: 0.0,
        edge_weight_type: String::new(),
        node_coords: Vec::new(),
        items: Vec::new(),
    };

    let reader = BufReader::new(file);
    let mut section = String::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return None,
        };

        if line.is_empty() {
            continue;
        }

        if line.starts_with("NODE_COORD_SECTION") {
            section = "NODE_COORD".to_string();
            continue;
        }

        if line.starts_with("ITEMS SECTION") {
            section = "ITEMS".to_string();
            continue;
        }

        if section == "NODE_COORD" {
            let node_info: Vec<&str> = line.split_whitespace().collect();
            if node_info.len() == 3 {
                let index = node_info[0].parse().unwrap_or(0);
                let x = node_info[1].parse().unwrap_or(0.0);
                let y = node_info[2].parse().unwrap_or(0.0);
                instance.node_coords.push((index, x, y));
            }
        } else if section == "ITEMS" {
            let item_info: Vec<&str> = line.split_whitespace().collect();
            if item_info.len() == 4 {
                let index = item_info[0].parse().unwrap_or(0);
                let profit = item_info[1].parse().unwrap_or(0.0);
                let weight = item_info[2].parse().unwrap_or(0.0);
                let assigned_node_number = item_info[3].parse().unwrap_or(0);
                instance
                    .items
                    .push((index, profit, weight, assigned_node_number));
            }
        }

        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0];
        let value = parts[1];

        match key {
            "PROBLEM NAME" => instance.problem_name = value.to_string(),
            "KNAPSACK DATA TYPE" => instance.knapsack_data_type = value.to_string(),
            "DIMENSION" => instance.dimension = value.parse().unwrap_or(0),
            "NUMBER OF ITEMS" => instance.num_items = value.parse().unwrap_or(0),
            "CAPACITY OF KNAPSACK" => instance.capacity_of_knapsack = value.parse().unwrap_or(0.0),
            "MIN SPEED" => instance.min_speed = value.parse().unwrap_or(0.0),
            "MAX SPEED" => instance.max_speed = value.parse().unwrap_or(0.0),
            "RENTING RATIO" => instance.renting_ratio = value.parse().unwrap_or(0.0),
            "EDGE_WEIGHT_TYPE" => instance.edge_weight_type = value.to_string(),
            _ => {
                eprint!("Unknown key: {}", key);
            }
        }
    }

    Some(instance)
}

fn main() {
    match parse_instance("./instances/a280_n279_bounded-strongly-corr_01.ttp") {
        Some(instance) => {
            solve_traveling_thief(&instance);
        }
        None => {
            println!("Failed to parse instance");
        }
    }
}
