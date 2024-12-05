use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn main() {}

fn parse_input(file_name: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let lines = std::fs::read_to_string(file_name).unwrap();
    let mut reached_empty = false;

    let mut rules: Vec<(u64, u64)> = Vec::new();

    let mut pages: Vec<Vec<u64>> = Vec::new();
    for line in lines.lines() {
        if line.is_empty() {
            reached_empty = true;
            continue;
        }

        if !reached_empty {
            let nums = line
                .split("|")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>();
            assert_eq!(nums.len(), 2);
            rules.push((nums[0], nums[1]));
        } else {
            let nums = line
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>();
            pages.push(nums);
        }
    }
    (rules, pages)
}

struct Checker {
    m_left: HashMap<u64, HashSet<u64>>,
    m_right: HashMap<u64, HashSet<u64>>,
}

impl Checker {
    fn check_line(&self, page: &Vec<u64>) -> Option<u64> {
        for (i1, n1) in page.iter().enumerate() {
            for (i2, n2) in page.iter().enumerate().skip(i1 + 1) {
                let r = self.m_left.get(n1)?;
                if !r.contains(n2) {
                    return None;
                }
            }
        }
        Some(page[page.len() / 2])
    }

    fn fix_order(&self, page: &mut Vec<u64>) {
        println!("\nfixeing: {:?}", page);
        page.sort_by(|a, b| {
            //
            println!("soring {},{}", a, b);
            let order = if let Some(av) = self.m_left.get(a) {
                println!("left contains {}", a);
                if av.contains(b) {
                    Ordering::Less
                } else if a == b {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            } else {
                println!("right contains {}", a);
                let bv = &self.m_right[a];
                if bv.contains(b) {
                    Ordering::Greater
                } else if a == b {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            };
            println!("{} {:?} {}", a, order, b);
            order
        });
        println!("fixed:{:?}", page)
    }
}

fn part1(file: &str) {
    let (rules, pages): (Vec<(u64, u64)>, Vec<Vec<u64>>) = parse_input(file);
    let mut m_left: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut m_right: HashMap<u64, HashSet<u64>> = HashMap::new();
    rules.iter().for_each(|&(left, right)| {
        m_left.entry(left).or_insert(HashSet::new()).insert(right);
        m_right.entry(right).or_insert(HashSet::new()).insert(left);
    });

    let checker = Checker { m_left, m_right };

    let result: u64 = pages
        .iter()
        .map(|page| checker.check_line(page))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .sum();
    println!("result: {result}")
}

fn part2(file: &str) {
    let (rules, pages): (Vec<(u64, u64)>, Vec<Vec<u64>>) = parse_input(file);
    let mut m_left: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut m_right: HashMap<u64, HashSet<u64>> = HashMap::new();
    rules.iter().for_each(|&(left, right)| {
        m_left.entry(left).or_insert(HashSet::new()).insert(right);
        m_right.entry(right).or_insert(HashSet::new()).insert(left);
    });

    let checker = Checker { m_left, m_right };

    let result: u64 = pages
        .iter()
        .map(|page| {
            //
            let result = checker.check_line(&page);
            if result.is_some() {
                return None;
            } else {
                let mut page = page.clone();
                checker.fix_order(&mut page);
                checker.check_line(&page)
            }
        })
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .sum();
    println!("result: {result}")
}

#[test]
fn part1_example() {
    part1("input.txt.example");
}

#[test]
fn part1_test() {
    part1("input.txt");
}

#[test]
fn part2_example() {
    part2("input.txt.example");
}

#[test]
fn part2_test() {
    part2("input.txt");
}
