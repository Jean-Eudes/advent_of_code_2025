use minifb::{Key, Window, WindowOptions};
use std::fs::read_to_string;

#[derive(Debug, Clone)]
#[derive(PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

struct Board {
    max_x: usize,
    max_y: usize,
    min_x: usize,
    min_y: usize,
    points: Vec<Point>,
}

impl Board {
    fn new(points: Vec<Point>, max_x: usize, max_y: usize, min_x: usize, min_y: usize) -> Self {
        Self {
            points,
            max_x,
            max_y,
            min_x,
            min_y,
        }

    }

    fn has_point_lower_left(&self, x: i64, y: i64) -> bool {
        self.points
            .iter()
            .filter(|point| point.x >= x && point.y <= y)
            .count()
            == 1
    }

    fn has_point_upper_left(&self, x: i64, y: i64) -> bool {
        self.points
            .iter()
            .filter(|point| point.x <= x && point.y <= y)
            .count()
            == 1
    }

    fn has_point_upper_right(&self, x: i64, y: i64) -> bool {
        self.points
            .iter()
            .filter(|point| point.x <= x && point.y >= y)
            .count()
            == 1
    }
    fn has_point_lower_right(&self, x: i64, y: i64) -> bool {
        self.points
            .iter()
            .filter(|point| point.x >= x && point.y >= y)
            .count()
            == 1
    }

    fn contains(&self, point: &Point) -> bool {
        self.has_point_lower_right(point.x, point.y)
            && self.has_point_upper_left(point.x, point.y)
            && self.has_point_lower_left(point.x, point.y)
            && self.has_point_upper_right(point.x, point.y)
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
    let string = read_to_string("exo9/example/example1.txt").unwrap();

    let mut points = Vec::new();

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;

    for line in string.lines() {
        let split = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        max_x = max_x.max(split[0]);
        max_y = max_y.max(split[1]);

        min_x = min_x.min(split[0]);
        min_y = min_y.min(split[1]);
        points.push(Point {
            x: split[0] as i64,
            y: split[1] as i64,
        });
    }

    let board = Board::new(points.clone(), max_x, max_y, min_x, min_y);

    println!("board size: {}, {}, {}, {}", max_x, max_y, min_x, min_y);

    const SCALE: usize = 50;

    let mut buffer: Vec<u32> = vec![0; 30 * 30];

    for edge in board.edges() {
        println!("edge: {:?}", edge);
        if edge.0.x == edge.1.x {
            for y in edge.0.y.min(edge.1.y)..=edge.1.y.max(edge.0.y) {
                buffer[y as usize * 30 + edge.1.x as usize] = 0xff0000;
            }
        }
        if edge.0.y == edge.1.y {
            for x in edge.0.x.min(edge.1.x)..=edge.1.x.max(edge.0.x) {
                buffer[edge.0.y as usize * 30 + x as usize] = 0xff0000;
            }
        }
        // buffer[point.y as usize * 30 + point.x as usize] = 0xff0000;
        //println!("point: {}, {}", point.x, point.y);
    }

    /*    for point in points {
            buffer[point.y as usize * 30 + point.x as usize] = 0xff0000;
        }
    */

    println!("buffer size: {:?}", buffer);

    println!("board size: {}, {}, {}, {}", max_x, max_y, min_x, min_y);

    let mut max_area = 0;

    let mut accepted = 0;
    let mut refused = 0;

    let mut final_point1 = Point { x: 0, y: 0 };
    let mut final_point2: Point = Point { x: 0, y: 0 };

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

                let included = points.iter().any(|p| p.x <= point_top_left.x && p.y <= point_top_left.y )
                    && points.iter().any(|p| p.x >= point_bottom_right.x && p.y >= point_bottom_right.y)
                    && points.iter().any(|p| p.x <= point_bottom_left.x && p.y >= point_bottom_left.y)
                    && points.iter().any(|p| p.x >= point_top_right.x && p.y <= point_top_right.y);

                //println!("hasedge : {has_edge}");
                if !has_point_in_rectangle && included && !has_edge {
                    max_area = area;
                    println!("accepted: {:?} {:?} {}", point1, point2, area);
                    final_point1 = point1.clone();
                    final_point2 = point2.clone();
                    accepted += 1;
                } else {
                    refused += 1;
                }
            }
        }
    }
    buffer[final_point1.y as usize * 30 + final_point1.x as usize] = 0x00ff00;
    buffer[final_point2.y as usize * 30 + final_point2.x as usize] = 0x00ff00;

    println!("accepted: {} refused: {}", accepted, refused);
    println!("area is: {:?}", max_area);
    println!("edges size is: {:?}", board.edges().len());

    let mut window = Window::new(
        "Test - ESC to exit",
        30 * SCALE,
        30 * SCALE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            // *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, 30, 30).unwrap();
    }
}

// 4650063000
// 4647960552
// 4647960552
// 2078252424
// 137900
// 2937935544
// 2315641564
// 2328873050
// 2220115874
// 1467575648
