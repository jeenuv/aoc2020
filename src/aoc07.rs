use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Rules {
    ascending: HashMap<String, HashSet<(usize, String)>>,
    descending: HashMap<String, HashSet<(usize, String)>>,
}

fn pick_bag_color(iter: &mut dyn Iterator<Item = &str>) -> String {
    iter.take(2).collect::<Vec<_>>().join(" ")
}

fn parse_rule(rules: &mut Rules, rule: String) {
    let mut bag_rules = rule.split_whitespace();
    let outer_bag = pick_bag_color(bag_rules.by_ref());

    assert_eq!(bag_rules.next().unwrap(), "bags");
    assert_eq!(bag_rules.next().unwrap(), "contain");

    if rule.ends_with("contain no other bags.") {
        // This is one of the smallest bags
        rules
            .descending
            .entry(outer_bag.clone())
            .or_insert(HashSet::new());
        return;
    }

    let inner_rules = bag_rules.collect::<Vec<_>>().join(" ");
    inner_rules.split(",").for_each(|r| {
        let mut inner_rule = r.split_whitespace();
        let cap = inner_rule.next().unwrap().parse::<usize>().unwrap();
        let inner_bag = pick_bag_color(&mut inner_rule);

        rules
            .ascending
            .entry(inner_bag.clone())
            .or_insert(HashSet::new())
            .insert((cap, outer_bag.clone()));

        rules
            .descending
            .entry(outer_bag.clone())
            .or_insert(HashSet::new())
            .insert((cap, inner_bag.clone()));

        assert_ne!(outer_bag, inner_bag);
    });
}

fn store_all_colors<'a>(path: &Vec<&'a (usize, String)>, all_colors: &RefCell<HashSet<&'a str>>) {
    let colors_in_path: HashSet<&str> = path.iter().map(|&(_, ref bag)| bag.as_str()).collect();
    let new_colors: HashSet<&str> = all_colors
        .borrow()
        .union(&colors_in_path)
        .copied()
        .collect();
    all_colors.replace(new_colors);
}

fn pack_for_ever<'a>(
    rules: &'a HashMap<String, HashSet<(usize, String)>>,
    bag: &'a str,
    scratch: &'a RefCell<Vec<&'a (usize, String)>>,
    cb: &dyn Fn(&RefCell<Vec<&'a (usize, String)>>),
) {
    match rules.get(bag) {
        Some(targets) => {
            for tgt in targets.iter() {
                scratch.borrow_mut().push(tgt);
                pack_for_ever(&rules, &tgt.1, scratch, cb);
                scratch.borrow_mut().pop();
            }
        }
        None => cb(scratch),
    }
}

fn part1(rules: &Rules) {
    let scratch = RefCell::new(Vec::new());
    let all_colors = RefCell::new(HashSet::new());
    pack_for_ever(&rules.ascending, "shiny gold", &scratch, &|v| {
        store_all_colors(&v.borrow(), &all_colors)
    });

    println!("part1: {:?}", all_colors.borrow().len() - 1);
}

fn bag_contribution(rules: &HashMap<String, HashSet<(usize, String)>>, bag: &str) -> usize {
    1_usize
        + rules[bag]
            .iter()
            .map(|(count, inner_bag)| count * bag_contribution(rules, inner_bag))
            .sum::<usize>()
}

fn part2(rules: &Rules) {
    println!(
        "part2: {:?}",
        bag_contribution(&rules.descending, "shiny gold") - 1
    );
}

pub fn run() {
    let br = BufReader::new(File::open("src/aoc07_input.txt").unwrap());

    let mut rules = Rules {
        ascending: HashMap::new(),
        descending: HashMap::new(),
    };
    br.lines()
        .map(|l| l.unwrap())
        .for_each(|l| parse_rule(&mut rules, l));

    part1(&rules);
    part2(&rules);
}
