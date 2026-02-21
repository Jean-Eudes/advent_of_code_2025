use std::collections::HashMap;
use std::fs::read_to_string;

struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, a: &'a str, b: &'a str) {
        self.edges.entry(a).or_insert_with(Vec::new).push(b);
    }
}

fn main() {
    let string = read_to_string("exo11/example/example2.txt").unwrap();
    let mut graph = Graph::new();

    for line in string.lines() {
        let (root, edges) = line.split_once(":").unwrap();
        let split = edges.trim().split_whitespace();
        for edge in split {
            graph.add_edge(root, edge);
        }
    }

    let number = count_path(&graph, "you");
    println!("result: {}", number);
}

fn count_path(graph: &Graph, start_node: &str) -> usize {
    if let Some(edges) = graph.edges.get(start_node) {
        let mut result = 0;
        for edge in edges {
            if *edge == "out" {
                result += 1;
            }
            result += count_path(graph, edge);
        }

        return result;
    }
    0
}
