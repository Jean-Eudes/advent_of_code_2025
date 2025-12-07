use std::collections::HashSet;
use std::fs::read_to_string;

enum Field {
    Start,
    Empty,
    Splitter,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '.' => Field::Empty,
            '^' => Field::Splitter,
            'S' => Field::Start,
            _ => panic!("Invalid field value: {}", value),
        }
    }
}

fn main() {
    let string = read_to_string("exo7/example/example2.txt").unwrap();

    let mut number_of_split: u64 = 0;

    let mut beam = HashSet::new();

    for line in string.lines() {

        let mut next_beam = HashSet::new();

        for (position, char) in line.chars().enumerate() {
            let field = Field::from(char);
            match field {
                Field::Start => {
                    next_beam.insert(position);
                    continue;
                }
                Field::Empty => {
                    if beam.contains(&position) {
                        next_beam.insert(position);
                    }
                }
                Field::Splitter => {
                    if beam.contains(&position) {
                        next_beam.insert(position - 1);
                        next_beam.insert(position + 1);
                        number_of_split += 1;
                    }
                }
            }
        }
        beam = next_beam;
    }

    println!("result: {number_of_split}");
}
