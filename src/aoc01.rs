use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_tuple(s: &HashSet<u32>, sum: u32) -> Vec<Vec<u32>> {
    let mut v = Vec::new();
    s.iter().for_each(|n| {
        if sum >= *n {
            if let Some(&o) = s.get(&(sum - *n)) {
                v.push(vec![*n, o]);
            }
        }
    });

    v
}

fn find_triple(s: &HashSet<u32>, sum: u32) -> Vec<Vec<u32>> {
    let mut v = Vec::new();
    for n in s {
        if sum < *n {
            continue;
        }

        let tuples = find_tuple(s, sum - *n);
        if tuples.is_empty() {
            continue;
        }

        for t in tuples {
            v.push(vec![*n, t[0], t[1]]);
        }
    }

    v
}

pub fn run() {
    let br = BufReader::new(File::open("src/aoc01_input.txt").unwrap());

    let numbers: HashSet<u32> = br
        .lines()
        .map(|l| u32::from_str_radix(&l.unwrap(), 10).unwrap())
        .collect();

    for t in find_tuple(&numbers, 2020) {
        println!("{:?} - product {}", t, t.iter().product::<u32>());
    }

    for t in find_triple(&numbers, 2020) {
        println!("{:?} - product {}", t, t.iter().product::<u32>());
    }
}
