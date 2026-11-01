use std::{env, process};
use csr_checker::{DependencyGraph};

fn main() {
    let args: Vec<String> = env::args().collect();

    let schedule = match args.get(1) {
        Some(s) => s,
        None => {
            eprintln!("No valid schedule was provided");
            process::exit(1);
        }
    };

    let mut dependency_graph = DependencyGraph::build(schedule).unwrap_or_else(|err| {
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