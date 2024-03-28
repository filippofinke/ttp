#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Path {
    pub nodes: Vec<(i32, i32, i32)>,
}

impl Path {
    pub fn new(nodes: Vec<(i32, i32, i32)>) -> Path {
        Path { nodes }
    }

    pub fn length(&self) -> f64 {
        let mut len = 0.0;
        for i in 0..(self.nodes.len() - 1) {
            let (_, x1, y1) = self.nodes[i];
            let (_, x2, y2) = self.nodes[i + 1];
            let dx = x2 - x1;
            let dy = y2 - y1;
            len += ((dx * dx + dy * dy) as f64).sqrt();
        }

        let (_, x1, y1) = self.nodes[self.nodes.len() - 1];
        let (_, x2, y2) = self.nodes[0];
        let dx = x2 - x1;
        let dy = y2 - y1;
        len += ((dx * dx + dy * dy) as f64).sqrt();
        len
    }
}
