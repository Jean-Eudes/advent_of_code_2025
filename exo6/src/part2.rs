use std::fs::read_to_string;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
            _ => panic!("Invalid operator"),
        }
    }
}

fn main() {
    let mut number_of_column = 0;
    let mut number_of_line = 0;

    let mut all_characters: Vec<char> = vec![];

    let string = read_to_string("exo6/example/example2.txt").unwrap();
    for line in string.lines() {
        number_of_column = line.len();
        number_of_line += 1;
        all_characters.append(&mut line.chars().collect());
    }

    let mut result: i64 = 0;

    let mut columns = Vec::new();
    for column in (0..number_of_column).rev() {
        let mut number = String::new();
        for line in 0..number_of_line - 1 {
            if all_characters[line * number_of_column + column] != ' ' {
                number.push(all_characters[line * number_of_column + column]);
            }
        }
        let operator = all_characters[(number_of_line - 1) * number_of_column + column];

        if number.is_empty() {
            continue;
        }
        columns.push(number.parse::<i64>().unwrap());

        if operator != ' ' {
            let operator = Operator::from(operator);
            result += match operator {
                Operator::Add => {columns.iter().fold(0, |acc, x| acc + x) }
                Operator::Multiply => {columns.iter().fold(1, |acc, x| acc * x) }
            };
            columns.clear();
        } 
        

    }

    println!("result: {}", result);
}
