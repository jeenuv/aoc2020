use std::fs::File;
use std::io::{BufRead, BufReader};

struct Pos {
    x: usize,
    y: usize,
    tile_width: usize,
    move_down: usize,
    move_right: usize,
}

impl Pos {
    fn move_right(&mut self) {
        self.x = (self.x + self.move_right) % self.tile_width;
    }

    fn move_down(&mut self) {
        self.y += self.move_down;
    }

    fn next_move(&mut self) {
        self.move_right();
        self.move_down();
    }

    fn has_tree(&self, map: &Vec<Vec<char>>) -> bool {
        map[self.y][self.x] == '#'
    }

    fn new(tile_width: usize, move_right: usize, move_down: usize) -> Pos {
        Pos {
            x: 0,
            y: 0,
            tile_width,
            move_right,
            move_down,
        }
    }
}

fn do_walk(map: &Vec<Vec<char>>, mr: usize, md: usize) -> usize {
    let mut pos = Pos::new(map[0].len(), mr, md);
    let height = map.len();
    let mut trees = 0;

    while pos.y < height {
        if pos.has_tree(&map) {
            trees += 1
        }
        pos.next_move()
    }

    trees
}

pub fn run() {
    let br = BufReader::new(File::open("src/aoc03_input.txt").unwrap());
    let mut map: Vec<Vec<char>> = Vec::new();

    for l in br.lines() {
        map.push(l.unwrap().chars().collect());
    }

    let args = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let results = args
        .iter()
        .map(|(mr, md)| do_walk(&map, *mr, *md))
        .product::<usize>();
    println!("result: {:?}", results);
}
