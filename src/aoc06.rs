use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process(answers: &Vec<String>) -> usize {
    let answer_sets: Vec<HashSet<_>> = answers
        .iter()
        .map(|a| a.chars().collect::<HashSet<_>>())
        .collect();

    let mut set_iter = answer_sets.into_iter();
    let acc = set_iter.next().unwrap();
    set_iter
        .fold(acc, |a, s| {
            a.intersection(&s).copied().collect::<HashSet<char>>()
        })
        .len()
}

pub fn run() {
    let br = BufReader::new(File::open("src/aoc06_input.txt").unwrap());

    let mut answers = Vec::new();
    let ans_sum: usize = br
        .lines()
        .map(|l| l.unwrap())
        .chain(std::iter::once("".to_string()))
        .filter_map(|l| {
            if l.is_empty() && !answers.is_empty() {
                let count = process(&answers);
                answers.clear();
                Some(count)
            } else {
                answers.push(l);
                None
            }
        })
        .sum();
    println!("{:?}", ans_sum);
}
