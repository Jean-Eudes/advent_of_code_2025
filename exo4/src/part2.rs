use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

#[derive(PartialEq)]
enum Field {
    Empty,
    Full,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Empty => write!(f, "."),
            Field::Full => write!(f, "@"),
        }
    }
}

struct Board {
    width: i64,
    height: i64,
    fields: Vec<Field>,
}

impl Board {
    fn new() -> Board {
        Board {
            width: 0,
            height: 0,
            fields: Vec::new(),
        }
    }

    fn add(&mut self, c: char) {
        self.fields.push(Field::from(c));
    }

    fn width(&mut self, width: i64) {
        self.width = width
    }
    fn height(&mut self, width: i64) {
        self.height = width
    }

    fn cell(&self, x: i64, y: i64) -> &Field {
        if x >= self.width || y >= self.fields.len() as i64 / self.width || x < 0 || y < 0 {
            return &Field::Empty;
        };
        &self.fields[x as usize + y as usize * self.width as usize]
    }

    fn retired(&mut self, x: i64, y: i64) {
        self.fields[x as usize + y as usize * self.width as usize] = Field::Empty;
    }

    fn neighbors(&self, x: i64, y: i64) -> usize {
        let mut number_of_neighbors = 0;

        if *self.cell(x - 1, y - 1) == Field::Full {
            number_of_neighbors += 1;
        }
        if *self.cell(x, y - 1) == Field::Full {
            number_of_neighbors += 1;
        }
        if *self.cell(x - 1, y) == Field::Full {
            number_of_neighbors += 1;
        }
        if *self.cell(x + 1, y) == Field::Full {
            number_of_neighbors += 1;
        }
        if *self.cell(x, y + 1) == Field::Full {
            number_of_neighbors += 1;
        }
        if *self.cell(x + 1, y + 1) == Field::Full {
            number_of_neighbors += 1;
        }
        if *self.cell(x - 1, y + 1) == Field::Full {
            number_of_neighbors += 1;
        }
        if *self.cell(x + 1, y - 1) == Field::Full {
            number_of_neighbors += 1;
        }

        number_of_neighbors
    }

    fn print(&self) {
        let mut x = 0;
        for field in self.fields.iter() {
            print!("{}", field);
            x += 1;
            if x == self.width {
                println!();
                x = 0;
            };
        }
    }
}

impl From<char> for Field {
    fn from(c: char) -> Field {
        match c {
            '.' => Field::Empty,
            '@' => Field::Full,
            _ => Field::Empty,
        }
    }
}

fn main() {
    let mut board = Board::new();

    let string = read_to_string("exo4/example/example2.txt").unwrap();

    let mut height = 0;

    for line in string.lines() {
        height += 1;
        board.width(line.len() as i64);
        for x in line.chars() {
            board.add(x);
        }
    }

    board.height(height);

    println!("{} {}", board.width, board.height);

    let mut result = 0;

    loop {
        let mut number_of_retired = 0;
        let mut retired = Vec::new();
        for y in 0..board.height {
            for x in 0..board.width {
                let i = board.neighbors(x, y);
                if i < 4 && *board.cell(x, y) == Field::Full {
                    retired.push((x, y));
                    number_of_retired += 1;
                }
            }
        }
        result += number_of_retired;
        if number_of_retired == 0 {
            break;
        }
        for cell in retired {
            board.retired(cell.0, cell.1);
        }
    }

    board.print();

    println!("result is {result:?}");
}
