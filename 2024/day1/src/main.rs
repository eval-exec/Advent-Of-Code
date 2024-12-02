use std::collections::HashMap;

fn parse_input() -> (Vec<u64>, Vec<u64>) {
    // parse input from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let a: u64 = iter.next().unwrap().parse().unwrap();
            let b: u64 = iter.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip()
}

fn part_2() {
    let (left, right) = parse_input();
    let mut right_map = HashMap::<u64, u64>::new();
    right.iter().for_each(|v| {
        let value = right_map.entry(*v).or_insert(0);
        *value += 1;
    });
    let result: u64 = left
        .iter()
        .map(|v| *v * right_map.get(v).unwrap_or(&0))
        .sum();
    println!("{result}")
}

fn part_1() {
    let (mut left, mut right) = parse_input();
    left.sort();
    right.sort();
    let result: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    println!("{result}");
}

fn main() {
    part_1();
    part_2();
}
