use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn index(&self) -> i64 {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

    fn turn_score(&self, new_direction: Direction) -> usize {
        let mut index = self.index() - new_direction.index();
        if index < 0 {
            index += 4;
        }
        if index == 2 {
            return 2000;
        }
        let score = (index % 2) as usize * 1000;
        score
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grid {
    Deer,
    Empty,

    Wall,
    End,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Deer {
    position: Position,
    direction: Direction,
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<Grid>>,
    deer: Deer,
    memo: HashMap<Position, usize>,
}

impl From<char> for Grid {
    fn from(value: char) -> Self {
        match value {
            '.' => Grid::Empty,
            'S' => Grid::Deer,
            'E' => Grid::End,
            '#' => Grid::Wall,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grid::Deer => f.write_str("S"),
            Grid::Empty => f.write_str("."),
            Grid::Wall => f.write_str("#"),
            Grid::End => f.write_str("E"),
        }
    }
}

fn parse_input(file_name: &str) -> Maze {
    let content = std::fs::read_to_string(file_name).unwrap();
    let map: Vec<Vec<Grid>> = content
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();
    let height = map.len();
    Maze {
        map,
        deer: Deer {
            position: Position(height - 2_usize, 1),
            direction: Direction::Right,
        },
        memo: HashMap::new(),
    }
}

impl Maze {
    fn print(&self, map: &Vec<Vec<Grid>>) {
        for (h, line) in map.iter().enumerate() {
            for (w, c) in line.iter().enumerate() {
                print!("{c}");
            }
            println!();
        }
        println!();
    }
    fn get_grid(&self, position: Position) -> Grid {
        self.map[position.0][position.1]
    }

    fn next_grid(&self, deer: Deer, direction: Direction) -> Grid {
        let next_p = self.next_position(deer.position, direction);
        self.get_grid(next_p)
    }

    // direction, count
    fn next_choidce(&self, deer: Deer) -> Vec<(Direction, usize)> {
        todo!()
    }

    fn next_position(&self, position: Position, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position(position.0 - 1, position.1),
            Direction::Down => Position(position.0 + 1, position.1),
            Direction::Left => Position(position.0, position.1 - 1),
            Direction::Right => Position(position.0, position.1 + 1),
        }
    }

    fn visit(&mut self, deer_position: Position) -> (Vec<Position>, usize) {
        let mut score = 0;
        let mut visited: HashMap<Position, bool> = HashMap::new();
        let mut min_score = usize::MAX;

        visited.insert(deer_position, true);

        println!("init: {:?}", self.deer);

        let mut min_score = usize::MAX;

        self.visit_inner(&mut visited, &Vec::new(), self.deer)
    }

    // direction, score.
    fn can_turn(&self, deer: Deer) -> Vec<(Direction, usize)> {
        let mut results = Vec::new();
        for next_direction in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            let next_grid = self.next_grid(deer, next_direction);
            if next_grid.eq(&Grid::Wall) {
                continue;
            }

            results.push((next_direction, deer.direction.turn_score(next_direction)));
        }
        results
    }

    fn visit_inner(
        &mut self,
        visited: &mut HashMap<Position, bool>,
        steps: &Vec<Position>,
        deer: Deer,
    ) -> (Vec<Position>, usize) {
        if let Some(&s) = self.memo.get(&deer.position) {
            println!("cache:{:?}:{}", deer.position, s);
            return (steps.to_owned(), s);
        }
        if self.get_grid(deer.position).eq(&Grid::End) {
            return (steps.to_owned(), 0);
        }

        let mut grid_score_path = Vec::new();
        let mut grid_score = usize::MAX;
        let next_directions: Vec<(Direction, usize)> = self
            .can_turn(deer)
            .into_iter()
            .filter(|(dic, _)| match deer.direction {
                Direction::Up => *dic != Direction::Down,
                Direction::Down => *dic != Direction::Up,
                Direction::Left => *dic != Direction::Right,
                Direction::Right => *dic != Direction::Left,
            })
            .collect::<Vec<_>>();
        println!("{deer:?}, got {:?}", next_directions);
        for (index, &(next_direction, turn_score)) in next_directions.iter().enumerate() {
            let next_p = self.next_position(deer.position, next_direction);
            if visited.contains_key(&next_p) {
                continue;
            }
            let next_deer = Deer {
                position: next_p,
                direction: next_direction,
            };
            let mut new_visited = visited.clone();
            let mut new_steps = steps.clone();
            new_steps.push(next_p);
            new_visited.insert(next_deer.position, true);
            let (this_steps, next_min_score) =
                self.visit_inner(&mut new_visited, &new_steps, next_deer);
            if next_min_score.eq(&usize::MAX) {
                continue;
            }
            if turn_score + 1 + next_min_score < grid_score {
                grid_score = turn_score + 1 + next_min_score;
                grid_score_path = this_steps;
            }
        }
        self.memo.insert(deer.position, grid_score);
        println!("inner: loop: {deer:?} : {grid_score}");
        (grid_score_path, grid_score)
    }
}

fn part1(file_name: &str) {
    println!();
    let mut maze = parse_input(file_name);
    let (path, min_score) = maze.visit(maze.deer.position);
    let mut map = maze.map.clone();
    path.iter().for_each(|position| {
        map[position.0][position.1] = Grid::Deer;
        // maze.print(&map);
    });
    println!("score: {}", min_score);
}

#[test]
fn part1_example() {
    part1("input.txt.example");
}

// #[test]
// fn part1_example1() {
//     part1("input.txt.example.1");
// }

// #[test]
// fn part1_main() {
//     part1("input.txt");
// }
