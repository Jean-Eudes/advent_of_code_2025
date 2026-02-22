use std::collections::HashMap;
use std::fs::read_to_string;

struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
}

impl Default for Graph<'_> {
    fn default() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }
}

impl<'a> Graph<'a> {
    fn add_edge(&mut self, a: &'a str, b: &'a str) {
        self.edges.entry(a).or_insert_with(Vec::new).push(b);
    }

    fn count_path(&self, start_node: &str) -> usize {
        if let Some(edges) = self.edges.get(start_node) {
            let mut result = 0;
            for edge in edges {
                if *edge == "out" {
                    result += 1;
                }
                result += self.count_path(edge);
            }
            return result;
        }
        0
    }
}

fn main() {
    let string = read_to_string("exo11/example/example2.txt").unwrap();
    let mut graph = Graph::default();

    for line in string.lines() {
        let (root, edges) = line.split_once(":").unwrap();
        let split = edges.trim().split_whitespace();
        for edge in split {
            graph.add_edge(root, edge);
        }
    }

    let number = graph.count_path("you");
    println!("result: {}", number);
}
