use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("📦 Generating dependency graph...");

    let mut graph = DiGraph::<&str, &str>::new();
    let mut nodes = HashMap::new();

    // Add modules
    let modules = vec!["main.rs", "utils.rs", "config.rs", "log.rs"];
    // Insert nodes and store references
    for &module in &modules {
        let node = graph.add_node(module);
        nodes.insert(module, node);
    }
    // Define dependencies
    let dependencies = vec![
        ("main.rs", "utils.rs"),
        ("main.rs", "config.rs"),
        ("utils.rs", "log.rs"),
    ];
    for (from, to) in dependencies {
        graph.add_edge(nodes[from], nodes[to], "depends on");
    }
    // Export to DOT
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    let mut file = File::create("dep_graph.dot").expect("Failed to write DOT file");
    write!(file, "{:?}", dot).expect("Failed to write content");

    println!("✅ Graph written to dep_graph.dot");
    println!("🖼️ Use 'dot -Tpng dep_graph.dot -o graph.png' to render.");
}
