use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x = ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
            as f64;
        x.sqrt()
    }
}

#[derive(Debug, PartialEq)]
struct Edge<'a> {
    point1: &'a Point,
    point2: &'a Point,
}

impl<'a> Edge<'a> {
    fn distance(&self) -> f64 {
        self.point1.distance(self.point2)
    }
}

#[derive(Debug)]
struct Circuit<'a> {
    points: Vec<&'a Point>,
}

impl<'a> Circuit<'a> {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn add_edge(&mut self, edge: &'a Edge<'a>) {
        self.points.push(edge.point1);
        self.points.push(edge.point2);
    }

    fn contains(&self, edge: &Edge<'_>) -> bool {
        self.points
            .iter()
            .any(|e| *e == edge.point1 || *e == edge.point2)
    }

    fn num_points(&self) -> usize {
        let mut points = HashSet::new();
        for edge in &self.points {
            points.insert(edge);
        }
        points.len()
    }

    fn merge(&self, other: &Self) -> Self {
        let mut circuit = Circuit::new();
        circuit.points.extend(other.points.iter());
        circuit.points.extend(self.points.iter());
        circuit
    }
}

#[derive(Debug)]
struct Circuits<'a> {
    circuits: Vec<Circuit<'a>>,
}

impl<'a> Circuits<'a> {
    fn new() -> Self {
        Self {
            circuits: Vec::new(),
        }
    }

    fn contains(&self, edge: &Edge<'a>) -> bool {
        self.circuits.iter().any(|circuit| circuit.contains(edge))
    }

    fn add_edge(&mut self, edge: &'a Edge<'a>) {
        if !self.contains(edge) {
            let mut circuit = Circuit::new();
            circuit.add_edge(edge);
            self.circuits.push(circuit);
        } else {
            let mut circuit = self
                .circuits
                .iter_mut()
                .filter(|circuit| circuit.contains(edge))
                .fold(Circuit::new(), |circuit1, circuit2| {
                    circuit1.merge(circuit2)
                });
            self.circuits.retain(|circuit| !circuit.contains(edge));
            circuit.add_edge(edge);
            self.circuits.push(circuit);
        }
    }

    fn result(mut self) -> u64 {
        self.circuits.sort_by_key(|a| a.num_points());
        self.circuits.reverse();
        self.circuits
            .iter()
            .take(3)
            .fold(1, |acc, circuit| acc * circuit.num_points() as u64)
    }
}

fn main() {
    let string = read_to_string("exo8/example/example2.txt").unwrap();

    let mut points = Vec::new();
    let mut edges = Vec::new();
    let mut circuits = Circuits::new();

    for line in string.lines() {
        let split = line
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        points.push(Point {
            x: split[0],
            y: split[1],
            z: split[2],
        });
    }

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            edges.push(Edge {
                point1: &points[i],
                point2: &points[j],
            });
        }
    }

    edges.sort_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap());

    edges.iter().take(1000).for_each(|e| circuits.add_edge(e));
    println!("{:?}", circuits.result());
}
