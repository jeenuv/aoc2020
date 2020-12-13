use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Passport {
    content: Vec<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            content: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    fn wipe(&mut self) {
        self.content.clear()
    }

    fn collect(&mut self, field: String) {
        let mut new_content: Vec<String> = field
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        self.content.append(&mut new_content);
    }

    fn validate(&self, mandatory: &HashSet<&str>, eyecolor: &HashSet<&str>) -> bool {
        let content_hash: HashMap<&str, &str> = self
            .content
            .iter()
            .map(|f| {
                let v: Vec<_> = f.split(":").collect();
                (v[0], v[1])
            })
            .collect();

        let keyset: HashSet<&str> = content_hash.keys().copied().collect();
        let diff: HashSet<&str> = mandatory.difference(&keyset).copied().collect();
        if diff.len() != 0 {
            return false;
        }

        keyset.iter().all(|k| {
            content_hash.get(k).map_or(true, |v| match *k {
                "byr" => v.len() == 4 && (1920..=2002).contains(&v.parse::<usize>().unwrap()),
                "iyr" => v.len() == 4 && (2010..=2020).contains(&v.parse::<usize>().unwrap()),
                "eyr" => v.len() == 4 && (2020..=2030).contains(&v.parse::<usize>().unwrap()),
                "hgt" => {
                    let length = v[..v.len() - 2].parse::<usize>().unwrap();
                    if v.ends_with("cm") {
                        (150..=193).contains(&length)
                    } else if v.ends_with("in") {
                        (59..=76).contains(&length)
                    } else {
                        false
                    }
                }
                "hcl" => {
                    v.len() == 7
                        && v.starts_with("#")
                        && v.get(1..).unwrap().chars().all(|c| c.is_ascii_hexdigit())
                }
                "ecl" => eyecolor.contains(v),
                "pid" => v.parse::<usize>().is_ok() && v.len() == 9,
                _ => true,
            })
        })
    }
}

pub fn run() {
    let br = BufReader::new(File::open("src/aoc04_input.txt").unwrap());
    let mandatory: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .copied()
        .collect();
    let eyecolor: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .copied()
        .collect();

    let mut passport = Passport::new();
    let valid_passports = br
        .lines()
        .map(|l| l.unwrap())
        .chain(std::iter::once("".to_string()))
        .filter_map(|l| {
            if l.is_empty() && !passport.is_empty() {
                let v = passport.validate(&mandatory, &eyecolor);
                passport.wipe();
                if v {
                    Some(v)
                } else {
                    None
                }
            } else {
                passport.collect(l);
                None
            }
        })
        .count();

    println!("{:?}", valid_passports);
}
