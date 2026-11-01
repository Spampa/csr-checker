use std::process;
use csr_checker::{DependencyGraph};

fn main() {
    let mut dependency_graph = DependencyGraph::build("w0,w1,w2,w3,w0").unwrap_or_else(|err| {
        eprintln!("Error during graph creation: {}", err);
        process::exit(1);
    });

    let cycles = dependency_graph.find_cycles();
    dependency_graph.print(cycles.as_deref().unwrap_or(&[]));

    match cycles {
        None => {
            println!("No cycles found")
        },
        Some(cycles) => {
            println!("Cycles found:");
            for c in cycles {
                println!("{}", c);
            }
        }
    }
}