#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub num_visited_nodes: usize,
    pub num_visited_leaves: usize,
    pub num_compared_nodes: usize,
    pub num_compared_leaves: usize,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            num_visited_nodes: 0,
            num_visited_leaves: 0,
            num_compared_nodes: 0,
            num_compared_leaves: 0,
        }
    }
}
