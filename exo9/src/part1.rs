use std::fs::read_to_string;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
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

    let mut max_area = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = points[i].aire_rectangle(&points[j]);
            max_area = max_area.max(area);
        }
    }
    println!("aire is: {:?}", max_area);
}
