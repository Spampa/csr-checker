use std::fmt;

pub struct Node {
    pub node_id: usize,
    pub connections: Vec<(usize, bool)>,
}

impl Node {
    pub fn is_connected(&self, other: usize) -> bool {
        self.connections.contains(&(other, false))
    }
}

pub struct Graph {
    nodes: Vec<Node>,
}

pub struct Chain(Vec<usize>);

impl Chain {
    /// Edges of the cycle only, skipping the path that leads into it.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        let last = self.0[self.0.len() - 1];
        let start = self.0.iter().position(|n| *n == last).unwrap_or(0);
        self.0[start..].windows(2).map(|w| (w[0], w[1])).collect()
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let last = self.0[self.0.len() - 1];
        let mut find_first = false;

        for n in &self.0 {
            if *n == last && !find_first {
                find_first = true;
                write!(f, "{n}")?;
            } else if find_first {
                write!(f, " -> ")?;
                write!(f, "{n}")?;
            }
        }
        Ok(())
    }
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node(&mut self, connections: Vec<usize>) -> usize {
        let node_id = self.nodes.len();
        self.nodes.push(Node {
            node_id,
            connections: connections.into_iter().map(|c| (c, false)).collect(),
        });
        node_id
    }

    pub fn add_connection(&mut self, i: usize, connection: usize) {
        self.nodes[i].connections.push((connection, false));
    }

    pub fn get_node(&self, i: usize) -> Option<&Node> {
        self.nodes.get(i)
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn find_cycles(&mut self) -> Option<Vec<Chain>> {
        let mut cycles = vec![];
        for i in 0..self.nodes.len() {
            if let Some(chain) = self.check_cycles(i, &mut vec![i]) {
                cycles.push(chain);
            }
        }
        self.reset_visited();

        if !cycles.is_empty() {
            Some(cycles)
        } else {
            None
        }
    }

    fn reset_visited(&mut self) {
        for n in &mut self.nodes {
            for c in &mut n.connections {
                c.1 = false;
            }
        }
    }

    fn check_cycles(&mut self, node_index: usize, chain: &mut Vec<usize>) -> Option<Chain> {
        for k in 0..self.nodes[node_index].connections.len() {
            if self.nodes[node_index].connections[k].1 {
                // already visited
                continue;
            }

            let next = self.nodes[node_index].connections[k].0;

            if chain.contains(&next) {
                chain.push(next);
                return Some(Chain(chain.clone()));
            }

            self.nodes[node_index].connections[k].1 = true;

            match self.nodes.get(next) {
                Some(n) => {
                    chain.push(n.node_id);
                    if let Some(c) = self.check_cycles(next, chain) {
                        return Some(c);
                    }
                    chain.pop();
                }
                None => continue,
            }
        }
        None
    }
}
