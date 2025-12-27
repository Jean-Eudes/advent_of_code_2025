use std::fs::read_to_string;

fn main() {
    let mut numbers_to_parse = Vec::new();

    let string = read_to_string("exo3/example/example2.txt").unwrap();
    for line in string.lines() {
        numbers_to_parse.push(line);
    }

    let mut result = 0;

    let number_of_digit = 11;

    for number in numbers_to_parse {
        let mut numbers = Vec::with_capacity(number_of_digit);
        let mut number_work = number;
        println!("number_work is {number_work:?}");
        for digit in (0..=number_of_digit).rev() {
            println!("digit is {digit:?}");
            let vec = sort_string(&number_work[0..number_work.len() - digit]);
            let digit = vec[0];
            numbers.push(vec[0]);

            number_work = number_work.split_once(digit).unwrap().1;
        }
        let x = numbers.iter().collect::<String>();
        println!("x is {x:?}");
        result += x.parse::<u64>().unwrap();
    }
    println!("result is {result:?}");
}

fn sort_string(number_work: &str) -> Vec<char> {
    let mut vec = number_work.chars().collect::<Vec<char>>();
    vec.sort_by(|a, b| b.cmp(a));
    vec
}
