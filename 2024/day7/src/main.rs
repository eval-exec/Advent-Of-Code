use std::{cell::RefCell, collections::HashSet, rc::Rc};

fn main() {
    println!("Hello, world!");
}

fn parse_input(file_name: &str) -> Vec<(u64, Vec<u64>)> {
    let content = std::fs::read_to_string(&file_name).unwrap();
    content
        .lines()
        .map(|line| {
            let eqs: Vec<&str> = line.split(":").collect();
            assert_eq!(eqs.len(), 2);
            let result: u64 = eqs[0].parse().unwrap();
            let nums: Vec<u64> = eqs[1]
                .trim()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect();
            return (result, nums);
        })
        .collect()
}

#[derive(Hash, PartialEq, Eq)]
struct Key {
    result: u64,
    nums: Vec<u64>,
}

fn check_inner(imposible: Rc<RefCell<HashSet<Key>>>, result: u64, nums: &[u64]) -> bool {
    // println!("checking {result}, {:?}", nums);
    if nums.is_empty() {
        if result == 0 {
            return true;
        } else {
            return false;
        }
    }
    let key = Key {
        result,
        nums: nums.to_vec(),
    };

    if imposible.borrow().contains(&key) {
        return false;
    }

    // if nums.iter().map(|v| *v).sum::<u64>() > result {
    //     imposible.borrow_mut().insert(key);
    //     println!("{:?} -> {result} imposible", nums);
    //     return false;
    // }

    let &v = nums.last().unwrap();

    if result >= v && check_inner(imposible.clone(), result - v, &nums[0..nums.len() - 1]) {
        return true;
    }
    if result % v == 0 && check_inner(imposible.clone(), result / v, &nums[0..nums.len() - 1]) {
        return true;
    }
    return false;
}

fn check_inner2(left: u64, nums: &[u64], target: u64) -> bool {
    // println!("checking {result}, {:?}", nums);
    if nums.is_empty() {
        return left == target;
    }

    if check_inner2(left * nums[0], &nums[1..], target) {
        return true;
    }

    if check_inner2(left + nums[0], &nums[1..], target) {
        return true;
    }

    if check_inner2(concatenation(left, nums[0]), &nums[1..], target) {
        return true;
    }
    return false;
}

fn concatenation(n1: u64, n2: u64) -> u64 {
    let mut N2 = n2;
    let mut N1 = n1;
    while N2 / 10 > 0 {
        N2 /= 10;
        N1 *= 10;
    }
    N1 *= 10;
    N1 + n2
}

fn check(result: u64, nums: &[u64]) -> bool {
    // println!("checking {result}, {:?}", nums);
    let imposible = Rc::new(RefCell::new(HashSet::new()));
    check_inner(imposible, result, &nums)
}

fn part1(file_name: &str) -> u64 {
    let inputs = parse_input(file_name);
    // println!("{:?}", inputs);
    let sum: u64 = inputs
        .iter()
        .filter(|(result, nums)| check(*result, nums))
        .map(|&(result, _)| result)
        .sum();

    sum
}

fn part2(file_name: &str) -> u64 {
    let inputs = parse_input(file_name);
    // println!("{:?}", inputs);
    let sum: u64 = inputs
        .iter()
        .filter(|(result, nums)| {
            let r = check_inner2(nums[0], &nums[1..], *result);
            println!("{:?}, {}, => {}", nums, *result, r);
            r
        })
        .map(|&(result, _)| result)
        .sum();

    sum
}

#[test]
fn part2_main() {
    let sum = part2("input.txt");
    println!("sum: {sum}")
}

#[test]
fn part2_example() {
    let sum = part2("input.txt.example");
    println!("sum: {sum}")
}

// #[test]
// fn part1_example() {
//     let sum = part1("input.txt.example");
//     println!("sum: {sum}");
// }

// #[test]
// fn part1_main() {
//     let sum = part1("input.txt");
//     println!("sum: {sum}");
// }
