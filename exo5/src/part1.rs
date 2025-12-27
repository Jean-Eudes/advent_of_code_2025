use std::fs::read_to_string;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, x: u64) -> bool {
        x >= self.start && x <= self.end
    }
}

fn main() {
    let string = read_to_string("exo5/example/example2.txt").unwrap();

    let mut ranges = Vec::new();

    let mut part = 0;

    let mut result = 0;

    'outer: for line in string.lines() {
        if line == "" {
            part += 1;
            continue;
        }

        if part == 0 {
            let vec = line.split("-").collect::<Vec<&str>>();
            ranges.push(Range {
                start: vec[0].parse::<u64>().unwrap(),
                end: vec[1].parse::<u64>().unwrap(),
            });
        }
        if part == 1 {
            let item = line.parse::<u64>().unwrap();

            for range in &ranges {
                if range.contains(item) {
                    println!("{:?}", item);
                    result += 1;
                    continue 'outer;
                }
            }
        }
    }

    println!("{:?}", result);
}
