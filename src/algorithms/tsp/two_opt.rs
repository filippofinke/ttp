use crate::models::path::Path;

pub struct TwoOpt;

impl TwoOpt {
    pub fn solve(path: &Path) -> Path {
        let mut best_path = path.clone();
        let mut improved = true;

        while improved {
            improved = false;
            for i in 0..best_path.nodes.len() - 1 {
                for j in i + 1..best_path.nodes.len() {
                    let mut new_path = best_path.clone();
                    new_path.nodes[i..=j].reverse();
                    if new_path.length() < best_path.length() {
                        best_path = new_path;
                        improved = true;
                    }
                }
            }
        }

        best_path
    }
}
