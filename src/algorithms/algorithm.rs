/*
 * Created on Tue Mar 26 2024
 *
 * Copyright (c) 2024 Filippo Finke
 */

use crate::{instance::Instance, solution::Solution};

pub trait Algorithm {
    fn solve(&self, problem: &Instance) -> Vec<Solution>;
}
