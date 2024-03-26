/*
 * Created on Tue Mar 26 2024
 *
 * Copyright (c) 2024 Filippo Finke
 */

use std::fmt::Display;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
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

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TSP Tour: {:?}\n\
            Packing Plan: {:?}\n\
            FP: {}\n\
            FT: {}\n\
            FTRaw: {}\n\
            OB: {}\n\
            WEnd: {}\n\
            WEnd Used: {}\n\
            Computation Time: {}",
            self.tsp_tour,
            self.packing_plan,
            self.fp,
            self.ft,
            self.ftraw,
            self.ob,
            self.wend,
            self.wend_used,
            self.computation_time
        )
    }
}

impl Solution {
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

    #[allow(dead_code)]
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
