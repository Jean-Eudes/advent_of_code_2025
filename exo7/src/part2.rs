use std::collections::HashMap;
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

    let mut number_of_timeline: usize = 0;

    let mut beam = HashMap::new();

    for line in string.lines() {
        let mut next_beam = HashMap::new();

        for (position, char) in line.chars().enumerate() {
            let field = Field::from(char);
            match field {
                Field::Start => {
                    next_beam.insert(position, 1);
                    number_of_timeline += 1;
                    continue;
                }
                Field::Empty => {
                    // Je remet dans la position les rayons de la position précédente
                    if beam.contains_key(&position) {
                        next_beam
                            .entry(position)
                            .and_modify(|count| *count += beam[&position])
                            .or_insert(beam[&position]);
                    }
                }
                Field::Splitter => {
                    /*
                     Je mets dans les positions gauche et droite autant de rayon que celui de la position courante,
                     plus les rayons de la position courante.

                     Pour chaque rayon de la position courante, j'incrémente la timeline de 1.
                    */

                    if beam.contains_key(&position) {
                        let current_value = beam.get(&position).unwrap_or(&0);
                        next_beam
                            .entry(position - 1)
                            .and_modify(|count| *count += current_value)
                            .or_insert(*current_value);
                        next_beam
                            .entry(position + 1)
                            .and_modify(|count| *count += current_value)
                            .or_insert(*current_value);

                        number_of_timeline += current_value;
                    }
                }
            }
        }

        beam = next_beam;
    }

    println!("result: {number_of_timeline}");
}
