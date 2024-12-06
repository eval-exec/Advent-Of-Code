use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

fn parse_input(file_name: &str) -> Vec<Vec<char>> {
    let content = std::fs::read_to_string(&file_name).unwrap();
    content.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn part1_fn(map: Vec<Vec<char>>) {
    let init = find_init_pos(&map);
    let result = visit(&map, init);
    println!("result: {}", result);
}

fn visit(map: &Vec<Vec<char>>, mut pos: (usize, usize)) -> u64 {
    let mut visited = HashSet::new();
    println!("init: {:?}", pos);
    let mut direction = Direction::Up;
    loop {
        visited.insert(pos);
        let mut next = pos;
        match direction {
            Direction::Up => {
                if next.0 == 0 {
                    break;
                }
                next.0 -= 1;
            }
            Direction::Right => {
                if next.1 == map[0].len() - 1 {
                    break;
                }
                next.1 += 1;
            }
            Direction::Down => {
                if next.0 == map.len() - 1 {
                    break;
                }
                next.0 += 1;
            }
            Direction::Left => {
                if next.1 == 0 {
                    break;
                }
                next.1 -= 1;
            }
        }
        if map[next.0][next.1] == '#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            continue;
        }
        pos = next;
        println!("visit {:?}, {}", pos, visited.len());
    }
    visited.len() as u64
}

fn find_init_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.eq(&'^') {
                return (i, j);
            }
        }
    }
    unreachable!();
}

// #[test]
// fn part1_example() {
//     let map = parse_input("input.txt.example");
//     part1_fn(map);
// }

// #[test]
// fn part1() {
//     let map = parse_input("input.txt");
//     part1_fn(map);
// }

fn part2_fn(map: Vec<Vec<char>>) {
    let init = find_init_pos(&map);
    let result = visit_p2(&map, init);
    println!("result: {}", result);
}

fn dead_loop(map: &Vec<Vec<char>>, mut pos: (usize, usize)) -> bool {
    let init = pos;
    let mut visited: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
    let mut direction = Direction::Up;
    let mut count: u64 = 0;
    loop {
        {
            if !visited
                .entry(pos)
                .or_insert(HashSet::new())
                .insert(direction.clone())
            {
                // println!("deadloop found: {:?}, {:?}", pos, direction);
                return true;
            }
        }
        let mut next = pos;
        match direction {
            Direction::Up => {
                if next.0 == 0 {
                    break;
                }
                next.0 -= 1;
            }
            Direction::Right => {
                if next.1 == map[0].len() - 1 {
                    break;
                }
                next.1 += 1;
            }
            Direction::Down => {
                if next.0 == map.len() - 1 {
                    break;
                }
                next.0 += 1;
            }
            Direction::Left => {
                if next.1 == 0 {
                    break;
                }
                next.1 -= 1;
            }
        }
        if map[next.0][next.1] == '#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            continue;
        }
        pos = next;
        // println!("deadloop visit {:?}, {}", pos, visited.len());
    }

    false
}

fn visit_p2(map: &Vec<Vec<char>>, mut pos: (usize, usize)) -> u64 {
    let init = pos;
    let mut direction = Direction::Up;
    let mut count: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let mut next = pos;
        match direction {
            Direction::Up => {
                if next.0 == 0 {
                    break;
                }
                next.0 -= 1;
            }
            Direction::Right => {
                if next.1 == map[0].len() - 1 {
                    break;
                }
                next.1 += 1;
            }
            Direction::Down => {
                if next.0 == map.len() - 1 {
                    break;
                }
                next.0 += 1;
            }
            Direction::Left => {
                if next.1 == 0 {
                    break;
                }
                next.1 -= 1;
            }
        }
        if map[next.0][next.1] == '#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            continue;
        } else {
            let mut map_clone = map.clone();
            map_clone[next.0][next.1] = '#';
            // println!("checking deadloop at : {:?}", next);
            if dead_loop(&map_clone, init) {
                count.insert(next);
            }
        }
        pos = next;
        // println!("visit {:?}", pos);
    }
    count.len() as u64
}

#[test]
fn part2_example() {
    let map = parse_input("input.txt.example");
    part2_fn(map);
}

#[test]
fn part2() {
    let map = parse_input("input.txt");
    part2_fn(map);
}
