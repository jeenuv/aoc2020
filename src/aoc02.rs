use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

pub fn run() {
    let br = BufReader::new(File::open("src/aoc02_input.txt").unwrap());

    let mut valid_count1 = 0;
    let mut valid_count2 = 0;
    for l in br.lines() {
        let line = l.unwrap();
        if line.is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split_whitespace().collect();
        let (range, c, password) = (fields[0], fields[1], fields[2]);

        let min_max: Vec<&str> = range.split("-").collect();
        let (c1, c2) = (
            usize::from_str_radix(min_max[0], 10).unwrap(),
            usize::from_str_radix(min_max[1], 10).unwrap(),
        );

        let c = c.chars().nth(0).unwrap();

        // Part 1
        let ccount = password.chars().filter(|chr| *chr == c).count();
        if ccount >= c1 && ccount <= c2 {
            valid_count1 += 1;
        }

        // Part 2
        let index_set = HashSet::<usize>::from_iter(vec![c1 - 1, c2 - 1]);
        let ccount = password
            .chars()
            .enumerate()
            .filter_map(|(i, chr)| {
                if index_set.contains(&i) && c == chr {
                    Some(1)
                } else {
                    None
                }
            })
            .count();

        if ccount == 1 {
            valid_count2 += 1;
        }
    }

    println!("part1: valid count: {:?}", valid_count1);
    println!("part2: valid count: {:?}", valid_count2);
}
