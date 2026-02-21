use std::collections::HashMap;
use std::fs::read_to_string;

struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
    cache: HashMap<&'a str, usize>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        let map = HashMap::new();
        Graph {
            edges: map,
            cache: HashMap::new(),
        }
    }

    fn add_edge(&mut self, a: &'a str, b: &'a str) {
        self.edges.entry(a).or_insert_with(Vec::new).push(b);
    }

    fn count_path(&mut self, start_node: &'a str, end_node: &'a str) -> usize {
        if let Some(edges) = self.edges.get(start_node) {
            let mut result = 0;
            for edge in edges.clone() {
                if edge == end_node {
                    result += 1;
                    continue;
                }
                result += if let Some(&cached) = self.cache.get(edge) {
                    cached
                } else {
                    let number_path_children = self.count_path(edge, end_node);
                    self.cache.insert(edge, number_path_children);
                    number_path_children
                };
            }

            return result;
        }
        0
    }

    fn clear_cache(&mut self) {
        self.cache.clear();
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

    println!("--------------------------------");
    let number1 = graph.count_path("svr", "fft");
    graph.clear_cache();
    let number2 = graph.count_path("fft", "dac");
    graph.clear_cache();

    let number3 = graph.count_path("dac", "out");
    graph.clear_cache();

    let result1_final = number3 * number2 * number1;

    let number1 = graph.count_path("svr", "dac");
    graph.clear_cache();

    let number2 = graph.count_path("dac", "fft");
    graph.clear_cache();

    let number3 = graph.count_path("fft", "out");
    graph.clear_cache();

    let result2_final = number3 * number2 * number1;

    println!(
        "result final: {} sur {}",
        result1_final + result2_final,
        graph.count_path("svr", "out")
    );
}
