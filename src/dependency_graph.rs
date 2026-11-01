use crate::{
    graph::{Chain, Graph},
    parser,
    printer::{GraphPrinter, Printer},
};
use std::error::Error;

pub struct DependencyGraph {
    graph: Graph,
}

impl DependencyGraph {
    pub fn build(input: &str) -> Result<Self, Box<dyn Error>> {
        let blocks = parser::parse_blocks(input)?;
        Ok(Self {
            graph: Self::create_dependencies_graph(&blocks),
        })
    }

    fn create_dependencies_graph(blocks: &[parser::Block]) -> Graph {
        let mut graph = Graph::new();

        for (i, current) in blocks.iter().enumerate() {
            if graph.get_node(current.node_id).is_none() {
                graph.add_node(vec![]);
            }

            for other in blocks.iter().skip(i + 1) {
                if current.is_conflict(other)
                    && !graph
                        .get_node(current.node_id)
                        .unwrap()
                        .is_connected(other.node_id)
                {
                    graph.add_connection(current.node_id, other.node_id);
                }
            }
        }
        graph
    }

    pub fn find_cycles(&mut self) -> Option<Vec<Chain>> {
        self.graph.find_cycles()
    }

    pub fn print(&self, cycles: &[Chain]) {
        GraphPrinter::print_graph(&self.graph, cycles);
    }
}
