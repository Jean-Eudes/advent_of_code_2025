use std::fs::read_to_string;

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl From<String> for Rotation {
    fn from(value: String) -> Self {
        let mut chars = value.chars();
        let dir = chars.next().expect("rotation string cannot be empty");
        let number_str: String = chars.collect();
        let amount: i32 = number_str
            .parse()
            .expect("rotation amount must be a valid u8 number");

        match dir {
            'L' | 'l' => Rotation::Left(amount),
            'R' | 'r' => Rotation::Right(amount),
            _ => panic!("invalid rotation direction: expected 'L' or 'R'"),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut result = Vec::new();
    for line in read_to_string("exo1/example/example2.txt").unwrap().lines() {
        result.push(Rotation::from(line.to_string()));
    }
    //println!("{:#?}", result);

    let mut initial_pos: i32 = 50;
    let mut number_of_zero = 0;

    for rotation in result {
        initial_pos = match rotation {
            Rotation::Left(amount) => {
                let mut result = initial_pos - amount;
                if result <= 0 && initial_pos > 0 {
                    number_of_zero += (100 - result) / 100;
                }
                if result <= 0 && initial_pos == 0 {
                    number_of_zero += (100 - result) / 100 - 1;
                }

                if result <= 0 {
                    result = 100 + (result % 100);
                }
                if result == 100 {
                    result = 0;
                }

                println!("i = {initial_pos} {result} ({rotation:?})");
                println!("number of zero  = {number_of_zero}");
                result
            }
            Rotation::Right(amount) => {
                let mut result = initial_pos + amount;

                if result >= 100 {
                    number_of_zero += result / 100;
                }

                if result > 100 {
                    result = result % 100;
                }
                if result == 100 {
                    result = 0;
                }

                println!("i =  {initial_pos} {result} ({rotation:?})");
                println!("number of zero  = {number_of_zero}");
                result
            }
        };
    }

    println!("password is {number_of_zero}");
}
