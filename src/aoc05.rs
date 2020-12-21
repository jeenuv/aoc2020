use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn bisect_window(codes: &str, window_width: usize, bisect_chars: &[char]) -> usize {
    let mut width = window_width;
    let mut start = 0;
    let mut end = start + width - 1;
    for c in codes.chars() {
        width /= 2;
        if c == bisect_chars[0] {
            end = start + width - 1;
        } else if c == bisect_chars[1] {
            start = start + width;
        }
    }

    assert!(start == end);

    start
}

fn seat_number(bp: String) -> usize {
    let row_code = &bp[..7];
    let column_code = &bp[7..];

    (bisect_window(row_code, 128, &['F', 'B']) * 8) + bisect_window(column_code, 8, &['L', 'R'])
}

pub fn run() {
    let br = BufReader::new(File::open("src/aoc05_input.txt").unwrap());
    let seats: Vec<usize> = br.lines().map(|l| l.unwrap()).map(seat_number).collect();

    let mut occupied = [false; 128 * 8];
    seats.into_iter().for_each(|s| occupied[s] = true);

    let unoccupied_seats: Vec<usize> = occupied
        .iter()
        .enumerate()
        .filter_map(|(i, &o)| if !o { Some(i) } else { None })
        .collect();

    let my_seat = unoccupied_seats
        .chunks(8)
        .filter_map(|ch| {
            let row_occupied: HashSet<&usize> = ch.iter().collect();
            for &seat in &row_occupied {
                let before_occupied = if seat > &0 { !row_occupied.contains(&(seat - 1)) } else { false };
                let after_occupied = !row_occupied.contains(&(seat + 1));
                if before_occupied && after_occupied {
                    return Some(seat);
                }
            }

            None
        })
        .next()
        .unwrap();

    println!("{:?}", my_seat);
}
