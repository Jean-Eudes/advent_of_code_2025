use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn number_of_elements(&self) -> u64 {
        if self.end >= self.start {
            self.end - self.start + 1
        } else {
            0
        }
    }

    fn contains(&self, x: u64) -> bool {
        x >= self.start && x <= self.end
    }
}
fn main() {
    let string = read_to_string("exo5/example/example2.txt").unwrap();

    let mut ranges: Vec<Range> = Vec::new();

    for line in string.lines() {
        if line == "" {
            break;
        }

        let vec = line.split("-").collect::<Vec<&str>>();
        let new_range = Range {
            start: vec[0].parse::<u64>().unwrap(),
            end: vec[1].parse::<u64>().unwrap(),
        };

        ranges.push(new_range);
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut ranges_computed: Vec<Range> = Vec::new();
    let mut result: u64 = 0;

    println!("{:?}", ranges);

    for range in ranges.clone().iter_mut() {
        let mut restart_loop = true;

        while restart_loop {
            restart_loop = false;
            for old_range in &ranges_computed {
                if old_range.contains(range.start) {
                    range.start = old_range.end + 1;
                    restart_loop = true;
                    break;
                }
                if old_range.contains(range.end) {
                    range.end = old_range.start - 1;
                    restart_loop = true;
                    break;
                }
            }
        }
        println!("range is {:?}", range);
        result += range.number_of_elements();
        println!("result is {}, {}", result, range.number_of_elements());
        ranges_computed.push(range.clone());
    }

    println!("{:?}", result);
}
