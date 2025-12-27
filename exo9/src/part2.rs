use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

struct Board {
    points: Vec<Point>,
}

impl Board {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn edges(&self) -> Vec<(&Point, &Point)> {
        let mut edges = Vec::new();
        for i in 0..self.points.len() - 1 {
            edges.push((&self.points[i], &self.points[i + 1]));
        }
        edges.push((&self.points[self.points.len() - 1], &self.points[0]));
        edges
    }
}

impl Point {
    fn aire_rectangle(&self, point: &Point) -> i64 {
        ((point.x - self.x).abs() + 1) * ((point.y - self.y).abs() + 1)
    }
}

fn main() {
    let string = read_to_string("exo9/example/example2.txt").unwrap();

    let mut points = Vec::new();

    for line in string.lines() {
        let split = line
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        points.push(Point {
            x: split[0],
            y: split[1],
        });
    }

    let board = Board::new(points.clone());

    let mut max_area = 0;

    let mut accepted = 0;
    let mut refused = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let point1 = &points[i];
            let point2 = &points[j];

            let point_top_left = Point {
                x: point1.x.min(point2.x),
                y: point1.y.min(point2.y),
            };
            let point_top_right = Point {
                x: point1.x.max(point2.x),
                y: point1.y.min(point2.y),
            };
            let point_bottom_left = Point {
                x: point1.x.min(point2.x),
                y: point1.y.max(point2.y),
            };
            let point_bottom_right = Point {
                x: point1.x.max(point2.x),
                y: point1.y.max(point2.y),
            };

            let area = point1.aire_rectangle(point2);

            let edges = board.edges();

            if area > max_area {
                // edge entre point_top_left et point_top_right (ligne du haut)

                let has_edge = edges.iter().any(|edge| edge.0.x == edge.1.x && ((point_top_left.x + 1)..point_top_right.x).contains(&edge.0.x) && (edge.0.y.min(edge.1.y) +1.. edge.0.y.max(edge.1.y)).contains(&point_top_left.y)) ||

                                // ligne du bas
                                edges.iter().any(|edge| edge.0.x == edge.1.x && ((point_bottom_left.x + 1)..point_bottom_right.x).contains(&edge.0.x) && (edge.0.y.min(edge.1.y)+1 .. edge.0.y.max(edge.1.y)).contains(&point_bottom_right.y) ) ||

                                // ligne de gauche
                                edges.iter().any(|edge| edge.0.y == edge.1.y && ((point_top_left.y + 1)..point_bottom_left.y).contains(&edge.0.y)  && (edge.0.x.min(edge.1.x) +1 .. edge.0.x.max(edge.1.x)).contains(&point_bottom_left.x)) ||

                                // ligne de droite
                                edges.iter().any(|edge| edge.0.y == edge.1.y && ((point_top_right.y + 1 )..point_bottom_right.y).contains(&edge.0.y) && (edge.0.x.min(edge.1.x) +1 .. edge.0.x.max(edge.1.x)).contains(&point_bottom_right.x) );

                let has_point_in_rectangle = board.points.iter().any(|p| {
                    p.x > point_top_left.x
                        && p.y > point_top_left.y
                        && p.x < point_bottom_right.x
                        && p.y < point_bottom_right.y
                });

                let included = points
                    .iter()
                    .any(|p| p.x <= point_top_left.x && p.y <= point_top_left.y)
                    && points
                        .iter()
                        .any(|p| p.x >= point_bottom_right.x && p.y >= point_bottom_right.y)
                    && points
                        .iter()
                        .any(|p| p.x <= point_bottom_left.x && p.y >= point_bottom_left.y)
                    && points
                        .iter()
                        .any(|p| p.x >= point_top_right.x && p.y <= point_top_right.y);

                //println!("hasedge : {has_edge}");
                if !has_point_in_rectangle && included && !has_edge {
                    max_area = area;
                    accepted += 1;
                } else {
                    refused += 1;
                }
            }
        }
    }
    println!("accepted: {} refused: {}", accepted, refused);
    println!("area is: {:?}", max_area);
    println!("edges size is: {:?}", board.edges().len());
}
