/*
 * Created on Tue Mar 26 2024
 *
 * Copyright (c) 2024 Filippo Finke
 */

use std::fs::File;
use std::io::Write;

pub struct Solution {
    pub tsp_tour: Vec<usize>,
    pub packing_plan: Vec<usize>,
    pub fp: f64,
    pub ft: f64,
    pub ftraw: u64,
    pub ob: f64,
    pub wend: f64,
    pub wend_used: f64,
    pub computation_time: u64,
}

impl Solution {
    pub fn new(tsp_tour: Vec<usize>, packing_plan: Vec<usize>) -> Self {
        Solution {
            tsp_tour,
            packing_plan,
            fp: f64::NEG_INFINITY,
            ft: f64::INFINITY,
            ftraw: u64::MAX,
            ob: f64::NEG_INFINITY,
            wend: f64::INFINITY,
            wend_used: f64::INFINITY,
            computation_time: u64::MAX,
        }
    }

    pub fn reset(&mut self) {
        self.fp = f64::NEG_INFINITY;
        self.ft = f64::INFINITY;
        self.ftraw = u64::MAX;
        self.ob = f64::NEG_INFINITY;
        self.wend = f64::INFINITY;
        self.wend_used = f64::INFINITY;
        self.computation_time = u64::MAX;
    }

    pub fn print(&self) {
        println!(
            "{} {} {} {} {} {} {}",
            self.wend, self.wend_used, self.fp, self.ftraw, self.ft, self.ob, self.computation_time
        );
    }

    pub fn println(&self) {
        self.print();
        println!();
    }

    pub fn print_full(&self) {
        self.println();
        println!("tspTour {:?}", self.tsp_tour);
        println!("packingPlan {:?}", self.packing_plan);
    }

    pub fn get_objective(&self) -> f64 {
        self.ob
    }

    fn answer(&self) -> String {
        let mut tour_out = vec![0; self.tsp_tour.len() - 1];
        for i in 0..self.tsp_tour.len() - 1 {
            tour_out[i] = self.tsp_tour[i] + 1;
        }

        let items_per_city = self.packing_plan.len() / (self.tsp_tour.len() - 2);

        let mut packing_plan_list = Vec::new();
        let mut packing_plan_index = 0;
        for i in 1..self.tsp_tour.len() - 1 {
            let city = self.tsp_tour[i];
            for _ in 0..items_per_city {
                if self.packing_plan[packing_plan_index] == 1 {
                    let item_index = (i - 1) * items_per_city + (city - 1);
                    let item_index_from_1 = item_index + 1;
                    packing_plan_list.push(item_index_from_1);
                }
                packing_plan_index += 1;
            }
        }
        packing_plan_list.sort();

        let packing_out: Vec<String> = packing_plan_list.iter().map(|&x| x.to_string()).collect();
        format!("{:?}\n{:?}\n", tour_out, packing_out)
    }

    pub fn write_result(&self, title: &str) {
        if let Ok(mut file) = File::create(title) {
            if let Err(e) = file.write_all(self.answer().as_bytes()) {
                eprintln!("Error writing to file: {}", e);
            }
        } else {
            eprintln!("Error creating file: {}", title);
        }
    }
}
