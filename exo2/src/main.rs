use std::fs::read_to_string;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

trait Valid {
    fn is_valid(&self) -> bool;
}

/*impl Valid for i64 {
    fn is_valid(&self) -> bool {
        let string = self.to_string();
        let mut current_char = 0;
        let first_char = &string[0..string.len() / 2];
        let last_char = &string[string.len() / 2..string.len()];
        if last_char.eq(first_char) {
            return false;
        }
        current_char += 1;

        true
    }
}
*/
impl Valid for i64 {
    fn is_valid(&self) -> bool {
        let string = self.to_string();
        let mut nb_char = 1;

        while nb_char < string.len() {
            let mut iteration_valid = true;
            let first_char = &string[0..nb_char];
            let last_char = &string[nb_char..string.len()];
            let split = last_char.split(first_char);
            for x in split {
                iteration_valid = iteration_valid && x.eq("");
            }

            if iteration_valid {
                return false;
            }

            nb_char += 1;
        }
        true
    }
}

impl From<String> for Range {
    fn from(value: String) -> Self {
        let mut split = value.split("-");
        Range {
            start: split.next().unwrap().parse().unwrap(),
            end: split.next().unwrap().parse().unwrap(),
        }
    }
}

impl Range {
    fn all_invalid_id(&self) -> Vec<i64> {
        let mut vec1 = vec![];
        for t in self.start..=self.end {
            if !t.is_valid() {
                vec1.push(t);
            }
        }

        vec1
    }
}

fn main() {

    let mut result = Vec::new();
    for line in read_to_string("exo2/example/example2.txt").unwrap().lines() {
        let split = line.split(",");
        for x in split {
            println!("{x}");
            result.push(Range::from(x.to_string()));
        }
    }

    println!("result");
    let mut result2 = 0;

    for x in &result {
        let vec = x.all_invalid_id();
        result2 += vec.iter().sum::<i64>();
    }
    println!("result is : {result2}");

}
