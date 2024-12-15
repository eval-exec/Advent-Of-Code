fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
enum Grid {
    Wall,
    Empty,
    Box,
    Robot,
}

impl From<char> for Grid {
    fn from(value: char) -> Self {
        match value {
            '#' => Grid::Wall,
            '.' => Grid::Empty,
            'O' => Grid::Box,
            '@' => Grid::Robot,
            x => unreachable!("unreachable!!!:{}", x),
        }
    }
}

struct Map {
    map: Vec<Vec<Grid>>,
    robot_position: (usize, usize),
}

impl Map {
    fn print(&self) {
        println!();
        for (i, line) in self.map.iter().enumerate() {
            for (j, grid) in line.iter().enumerate() {
                print!("{grid:05?}");
            }
            println!();
        }
        println!();
    }
    fn grid(&self, position: (usize, usize)) -> Grid {
        self.map[position.0][position.1]
    }
    fn set_grid(&mut self, position: (usize, usize), grid: Grid) {
        self.map[position.0][position.1] = grid
    }
    fn robot_move(&mut self, instruction: Instruction) {
        let mut empty_position = self.robot_position;
        let mut next_grid = self.robot_position;
        let mut found_empty = false;
        match instruction {
            Instruction::Left => {
                next_grid.1 -= 1;
                while !matches!(self.map[empty_position.0][empty_position.1], Grid::Wall) {
                    empty_position.1 -= 1;
                    if matches!(self.grid(empty_position), Grid::Empty) {
                        found_empty = true;
                        break;
                    }
                }
            }
            Instruction::Up => {
                next_grid.0 -= 1;
                while !matches!(self.map[empty_position.0][empty_position.1], Grid::Wall) {
                    empty_position.0 -= 1;
                    if matches!(self.grid(empty_position), Grid::Empty) {
                        found_empty = true;
                        break;
                    }
                }
            }
            Instruction::Down => {
                next_grid.0 += 1;
                while !matches!(self.map[empty_position.0][empty_position.1], Grid::Wall) {
                    empty_position.0 += 1;
                    if matches!(self.grid(empty_position), Grid::Empty) {
                        found_empty = true;
                        break;
                    }
                }
            }
            Instruction::Right => {
                next_grid.1 += 1;
                while !matches!(self.map[empty_position.0][empty_position.1], Grid::Wall) {
                    empty_position.1 += 1;
                    if matches!(self.grid(empty_position), Grid::Empty) {
                        found_empty = true;
                        break;
                    }
                }
            }
        }

        if found_empty {
            self.set_grid(empty_position, Grid::Box);
            self.set_grid(self.robot_position, Grid::Empty);
            self.set_grid(next_grid, Grid::Robot);
            self.robot_position = next_grid;
        }
    }

    fn robot_moves(&mut self, instructions: Vec<Instruction>) {
        instructions
            .iter()
            .for_each(|&instruction| self.robot_move(instruction));
    }

    fn sum_gps(&self) -> usize {
        let mut sum = 0;
        for (height, line) in self.map.iter().enumerate() {
            for (width, grid) in line.iter().enumerate() {
                if matches!(grid, Grid::Box) {
                    sum += 100 * height + width;
                }
            }
        }
        sum
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            '>' => Instruction::Right,
            '<' => Instruction::Left,
            '^' => Instruction::Up,
            'v' => Instruction::Down,
            x => unreachable!("unreachable!!!:{}", x),
        }
    }
}

fn parse_input(file_name: &str) -> (Map, Vec<Instruction>) {
    let content = std::fs::read_to_string(file_name).unwrap();
    let splits: Vec<_> = content.split("\n\n").collect();
    assert_eq!(splits.len(), 2);
    let map: Vec<Vec<Grid>> = splits[0]
        .trim()
        .lines()
        .map(|line| line.chars().map(|char| char.into()).collect())
        .collect();
    let instruction = splits[1]
        .trim()
        .chars()
        .filter(|ch| !ch.eq(&'\n'))
        .map(|c| c.into())
        .collect();
    let find_robot = |map: &Vec<Vec<Grid>>| -> (usize, usize) {
        for (i, line) in map.iter().enumerate() {
            if let Some(j) = line.iter().position(|g| matches!(g, Grid::Robot)) {
                return (i, j);
            }
        }
        unreachable!();
    };
    let robot_position = find_robot(&map);

    (
        Map {
            map,
            robot_position,
        },
        instruction,
    )
}

fn part1(file_name: &str) {
    let (mut map, mut instructions) = parse_input(file_name);
    map.robot_moves(instructions);
    let sum = map.sum_gps();
    println!("sum: {sum}");
}

#[test]
fn part1_example() {
    part1("input.txt.larger");
}

#[test]
fn part1_small() {
    part1("input.txt.small");
}

#[test]
fn part1_main() {
    part1("input.txt");
}
