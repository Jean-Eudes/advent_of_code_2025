use std::fs::read_to_string;

fn main() {
    let mut numbers_to_parse = Vec::new();

    let string = read_to_string("exo3/example/example2.txt").unwrap();
    for line in string.lines() {
        numbers_to_parse.push(line);
    }

    let mut result = 0;

    for number in numbers_to_parse {
        let number_work = number.to_string();

        let vec = sort_string(&number_work[0..number_work.len() - 1]);
        let first_digit = vec[0];

        let second_part = number_work.split_once(first_digit).unwrap().1;
        let vec1 = sort_string(&second_part[0..]);
        let string1 = format!("{}{}", first_digit, vec1[0]);
        result += string1.parse::<u32>().unwrap();
    }
    println!("result is {result:?}");
}

fn sort_string(number_work: &str) -> Vec<char> {
    let mut vec = number_work.chars().collect::<Vec<char>>();
    vec.sort_by(|a, b| b.cmp(a));
    vec
}
