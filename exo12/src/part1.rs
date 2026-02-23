use std::fs::read_to_string;

struct Piece {
    white: isize,
    black: isize,
}

impl Piece {
    fn new(white: isize, black: isize) -> Piece {
        Piece { white, black }
    }
    fn signature(&self) -> isize {
        self.black - self.white
    }
}

struct Board {
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board { width, height }
    }

    fn signature(&self) -> isize {
        if self.width * self.height % 2 == 0 {
            0
        } else {
            1
        }
    }
}

fn main() {
    let string = read_to_string("exo12/example/example1.txt").unwrap();

    let number_pieces = 5;
    let counter_piece = 0;

    for i in 0..5 {
        let number_pieces = string.lines().count();
    }

    for line in string.lines() {
        let (root, edges) = line.split_once(":").unwrap();
        let split = edges.trim().split_whitespace();
        for edge in split {
        }
    }
}
