use std::collections::HashMap;
use std::fs::read_to_string;

struct Graph {
    nodes: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        let map = HashMap::new();
        Graph { nodes: map }
    }

    fn add_node(&mut self, a: String, b: String) {
        self.nodes.entry(a).or_insert_with(Vec::new).push(b);
    }
}

fn main() {
    let string = read_to_string("exo11/example/example2.txt").unwrap();
    let mut graph = Graph::new();

    for line in string.lines() {
        let root = line.split(":").next();
        let edges_opt = line.split(":").skip(1).next();
        let split = edges_opt.unwrap().trim().split(" ");
        for edge in split {
            graph.add_node(root.unwrap().to_string(), edge.to_string());
        }
    }
    println!("{:?}", graph.nodes);

    let number = traverse_graph(&graph, "you");
    println!("result: {}", number);

}

fn traverse_graph(graph: &Graph, start_node: &str) -> usize {

    if let Some(edges) = graph.nodes.get(start_node) {
        let mut result = 0;
        for edge in edges {
            if edge == "out" {
                result += 1;
            }
            result += traverse_graph(graph, edge);
        }

        return result;
    }
    0
}
