fn parse_input() -> Vec<Vec<u64>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

fn check(nums: &[u64]) -> bool {
    let is_increasing = nums[1] > nums[0];
    for (i, n) in nums.iter().enumerate().skip(1) {
        if is_increasing != (nums[i] > nums[i - 1]) {
            return false;
        }
        let abs = nums[i].abs_diff(nums[i - 1]);
        if abs < 1 || abs > 3 {
            return false;
        }
    }
    true
}

fn part_1() {
    let input = parse_input();
    let result = input.iter().filter(|line| check(line)).count();
    println!("{result}");
}

fn recheck(index: usize, nums: &[u64]) -> bool {
    let side_check = |&delta| -> bool {
        if index as i64 + delta >= 0 && index as i64 + delta < nums.len() as i64 {
            let mut nums = nums.to_vec();
            nums.remove((index as i64 + delta) as usize);
            return check(&nums);
        };
        false
    };

    vec![-2 as i64, -1, 0, 1]
        .iter()
        .map(|delta| side_check(delta))
        .filter(|is_true| *is_true)
        .count()
        > 0
}

fn check_dampener(nums: &[u64]) -> bool {
    let is_increasing = nums[1] > nums[0];
    for (i, n) in nums.iter().enumerate().skip(1) {
        if is_increasing != (nums[i] > nums[i - 1]) {
            return recheck(i, nums);
        }
        let abs = nums[i].abs_diff(nums[i - 1]);
        if abs < 1 || abs > 3 {
            return recheck(i, nums);
        }
    }
    true
}
fn part_2() {
    let input = parse_input();
    let result = input.iter().filter(|nums| check_dampener(nums)).count();
    println!("{result}")
}

fn main() {
    part_1();
    part_2();
}
