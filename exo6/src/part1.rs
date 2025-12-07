use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Invalid operator"),
        }
    }
}

fn main() {
    let mut number_of_column = 0;

    let mut numbers: Vec<&str> = vec![];
    let mut operators: Vec<Operator> = vec![];

    let string = read_to_string("exo6/example/example2.txt").unwrap();

    let separator = Regex::new(r"\s+").expect("Invalid regex");

    for line in string.lines() {
        if line.trim().starts_with("*") || line.trim().starts_with("+") {
            let indices: Vec<&str> = separator.split(line.trim()).collect();
            operators = indices
                .into_iter()
                .map(|x| Operator::from(x))
                .collect::<Vec<Operator>>();
            continue;
        }

        let mut indices: Vec<&str> = separator.split(line.trim()).collect();
        number_of_column = indices.len();
        numbers.append(&mut indices);
    }

    let mut result: i64 = 0;
    for column in 0..number_of_column {
        let operator = &operators[column];

        let mut intermediary_result = match operator {
            Operator::Add => 0,
            Operator::Multiply => 1,
        };

        for row in numbers.iter().skip(column).step_by(number_of_column) {
            match operator {
                Operator::Add => intermediary_result += row.parse::<i64>().unwrap(),
                Operator::Multiply => intermediary_result *= row.parse::<i64>().unwrap(),
            }
        }
        result += intermediary_result;
    }
    println!("result: {result}")
}
