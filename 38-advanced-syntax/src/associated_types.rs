pub trait Graph {
    type Node;
    type Edge;

    fn edges(&self, node: &Self::Node) -> Vec<Self::Edge>;
}

pub struct SimpleGraph {
    pub connections: Vec<(u32, u32)>,
}

impl Graph for SimpleGraph {
    type Node = u32;
    type Edge = (u32, u32);

    fn edges(&self, node: &Self::Node) -> Vec<Self::Edge> {
        self.connections
            .iter()
            .filter(|(from, _)| from == node)
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, SimpleGraph};

    #[test]
    fn implementation_selects_associated_types() {
        let graph = SimpleGraph {
            connections: vec![(1, 2), (1, 3), (2, 3)],
        };
        assert_eq!(graph.edges(&1), vec![(1, 2), (1, 3)]);
    }
}
