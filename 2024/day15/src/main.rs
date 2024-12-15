fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grid {
    Wall,
    Empty,
    BoxOld,
    BoxLeft,
    BoxRight,
    Robot,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grid::Wall => f.write_str("#"),
            Grid::Empty => f.write_str("."),
            Grid::BoxOld => f.write_str("O"),
            Grid::BoxLeft => f.write_str("["),
            Grid::BoxRight => f.write_str("]"),
            Grid::Robot => f.write_str("@"),
        }
    }
}

impl From<char> for Grid {
    fn from(value: char) -> Self {
        match value {
            '#' => Grid::Wall,
            '.' => Grid::Empty,
            'O' => Grid::BoxOld,
            '[' => Grid::BoxLeft,
            ']' => Grid::BoxRight,
            '@' => Grid::Robot,
            x => unreachable!("unreachable!!!:{}", x),
        }
    }
}

struct Map {
    map: Vec<Vec<Grid>>,
    robot_position: Position,
}

impl Map {
    fn scale(&mut self) {
        let line_len = self.map[0].len();
        for h in 0..self.map.len() {
            for i in 0..line_len {
                let dup = self.map[h][i * 2];
                match dup {
                    Grid::BoxOld => {
                        self.map[h][i * 2] = Grid::BoxLeft;
                        self.map[h].insert(i * 2 + 1, Grid::BoxRight);
                    }
                    Grid::Robot => {
                        self.map[h].insert(i * 2 + 1, Grid::Empty);
                    }
                    _ => {
                        self.map[h].insert(i * 2 + 1, dup);
                    }
                }
            }
        }
        self.robot_position.1 *= 2;
    }

    fn print(&self) {
        println!();
        for (i, line) in self.map.iter().enumerate() {
            for (j, grid) in line.iter().enumerate() {
                print!("{grid}");
            }
            println!();
        }
        println!();
    }
    fn grid(&self, position: Position) -> Grid {
        self.map[position.0][position.1]
    }
    fn set_grid(&mut self, position: Position, grid: Grid) {
        self.map[position.0][position.1] = grid
    }

    fn robot_try_move(
        &self,
        now_position: Position,
        instruction: Instruction,
    ) -> Option<Vec<BoxInstruction>> {
        let mut r1 = self.robot_try_move_inner(now_position, instruction)?;

        let pair_position = {
            match (self.grid(now_position), instruction) {
                (Grid::BoxLeft, Instruction::Up | Instruction::Down) => {
                    Some(now_position.next_position(Instruction::Right))
                }
                (Grid::BoxRight, Instruction::Up | Instruction::Down) => {
                    Some(now_position.next_position(Instruction::Left))
                }
                _ => None,
            }
        };

        if let Some(pair_position) = pair_position {
            println!(
                "{} have pair {}, {:?} have {:?}",
                self.grid(now_position),
                self.grid(pair_position),
                now_position,
                pair_position
            );
            let r2 = self.robot_try_move_inner(pair_position, instruction)?;
            for r in r2 {
                if !r1.contains(&r) {
                    r1.push(r);
                }
            }
            Some(r1)
        } else {
            Some(r1)
        }
    }

    fn robot_try_move_inner(
        &self,
        now_position: Position,
        instruction: Instruction,
    ) -> Option<Vec<BoxInstruction>> {
        let next_position = now_position.next_position(instruction);

        let this_instruction = BoxInstruction {
            now_position,
            next_position,
        };

        match self.grid(next_position) {
            Grid::Wall => return None,
            Grid::Empty => {
                // println!("try move inner one: {:?}", this_instruction);
                return Some(vec![this_instruction].into_iter().collect());
            }
            Grid::BoxLeft | Grid::BoxRight | Grid::BoxOld => {
                if let Some(mut child_instructions) =
                    self.robot_try_move(next_position, instruction)
                {
                    child_instructions.push(this_instruction);
                    // println!("try move inner multi: {:?}", child_instructions);
                    return Some(child_instructions.into());
                }
            }
            Grid::Robot => unreachable!(),
        }
        None
    }

    fn robot_move(&mut self, robot_instruction: Instruction) {
        println!("robot move: {:?}---------------------", robot_instruction);
        if let Some(box_instructions) = self.robot_try_move(self.robot_position, robot_instruction)
        {
            // box_instructions.iter().for_each(|i| {
            //     println!(" box_instructions: {:?}", i);
            // });
            box_instructions.iter().for_each(|box_instruction| {
                println!(
                    "{box_instruction:?}, now: {}, next: {}",
                    self.grid(box_instruction.now_position),
                    self.grid(box_instruction.next_position)
                );
                assert_eq!(self.grid(box_instruction.next_position), Grid::Empty);
                self.set_grid(
                    box_instruction.next_position,
                    self.grid(box_instruction.now_position),
                );
                self.set_grid(box_instruction.now_position, Grid::Empty);
            });

            self.set_grid(self.robot_position, Grid::Empty);
            let robot_next_position = self.robot_position.next_position(robot_instruction);
            self.set_grid(robot_next_position, Grid::Robot);
            self.robot_position = robot_next_position;
        }
    }

    fn robot_moves(&mut self, instructions: Vec<Instruction>) {
        instructions.iter().for_each(|&robot_instruction| {
            self.robot_move(robot_instruction);
        });
    }

    fn sum_gps(&self) -> usize {
        let mut sum = 0;
        for (height, line) in self.map.iter().enumerate() {
            for (width, grid) in line.iter().enumerate() {
                if matches!(grid, Grid::BoxLeft) {
                    sum += 100 * height + width;
                }
            }
        }
        sum
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position(usize, usize);

impl Position {
    fn next_position(&self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Up => Position(self.0 - 1_usize, self.1),
            Instruction::Down => Position(self.0 + 1_usize, self.1),
            Instruction::Left => Position(self.0, self.1 - 1_usize),
            Instruction::Right => Position(self.0, self.1 + 1_usize),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct BoxInstruction {
    now_position: Position,
    next_position: Position,
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
    let find_robot = |map: &Vec<Vec<Grid>>| -> Position {
        for (i, line) in map.iter().enumerate() {
            if let Some(j) = line.iter().position(|g| matches!(g, Grid::Robot)) {
                return Position(i, j);
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
    let (mut map, instructions) = parse_input(file_name);
    map.robot_moves(instructions);
    let sum = map.sum_gps();
    println!("sum: {sum}");
}

// running 3 tests
// test part1_example ... sum: 10092
// ok
// test part1_main ... sum: 1487337
// ok
// test part1_small ... sum: 2028
// ok
// #[test]
// fn part1_example() {
//     part1("input.txt.larger");
// }

// #[test]
// fn part1_small() {
//     part1("input.txt.small");
// }

// #[test]
// fn part1_main() {
//     part1("input.txt");
// }

fn part2(file_name: &str) {
    let (mut map, instructions) = parse_input(file_name);
    map.scale();
    println!("robot position: {:?}", map.robot_position);
    map.print();
    println!("monving");
    map.robot_moves(instructions);
    let sum = map.sum_gps();
    println!("sum: {sum}");
}

#[test]
fn part2_example_large() {
    part2("input.txt.larger");
}
#[test]
fn part2_main() {
    part2("input.txt");
}
