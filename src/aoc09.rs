use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_invalid_number(br: &mut dyn BufRead, window: usize) {
    let mut deque = VecDeque::new();
    let mut num_set = HashSet::new();
    let numbers: Vec<isize> = br
        .lines()
        .map(|l| l.unwrap().parse::<isize>().unwrap())
        .collect();

    numbers.iter().take(window).for_each(|e| {
        deque.push_back(e);
        num_set.insert(e);
    });

    let mut key = 0;
    for num in numbers.iter().skip(window) {
        let mut found = false;
        for seen in num_set.iter() {
            if num_set.contains(&(num - *seen)) {
                found = true;
                break;
            }
        }

        if found {
            let goner = deque.pop_front().unwrap();
            num_set.remove(&goner);

            deque.push_back(num);
            num_set.insert(num);
        } else {
            key = *num;
            break;
        }
    }

    println!("part1: {:?}", key);

    let mut slice_width = 2;
    'outer: loop {
        if slice_width >= numbers.len() {
            break;
        }

        let mut start = 0;
        loop {
            let end = start + slice_width;
            if end > numbers.len() {
                break;
            }

            let num_slice = &numbers[start..end];
            if num_slice.iter().sum::<isize>() == key {
                println!(
                    "part2: {:?}",
                    num_slice.iter().min().unwrap() + num_slice.iter().max().unwrap()
                );
                break 'outer;
            }

            start += 1;
        }

        slice_width += 1;
    }
}

pub fn run() {
    let mut br = BufReader::new(File::open("src/aoc09_input.txt").unwrap());
    first_invalid_number(&mut br, 25);
}
