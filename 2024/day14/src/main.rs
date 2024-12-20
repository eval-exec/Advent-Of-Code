fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn run_1s(self, area: Area) -> Self {
        self.run_after(1, area)
    }

    fn run_after(self, after: u64, area: Area) -> Self {
        // println!("before {:?}", self.clone());
        //
        let Position { x, y } = self.position;
        let mut new_position = Position {
            x: (x + self.velocity.x * after as i64) % (area.wide),
            y: (y + self.velocity.y * after as i64) % (area.tall),
        };
        if new_position.x < 0 {
            new_position.x += area.wide;
        }
        if new_position.y < 0 {
            new_position.y += area.tall;
        }
        let robot = Self {
            position: new_position,
            velocity: self.velocity,
        };
        // println!("after {:?}", robot);
        robot
    }
}

fn parse_input(file_name: &str) -> Vec<Robot> {
    let content = std::fs::read_to_string(file_name).unwrap();
    content
        .lines()
        .map(|line| {
            let parts = line.split(&['=', ',', ' ']).collect::<Vec<_>>();
            let px: i64 = parts[1].parse().unwrap();
            let py: i64 = parts[2].parse().unwrap();
            let vx: i64 = parts[4].parse().unwrap();
            let vy: i64 = parts[5].parse().unwrap();
            Robot {
                position: Position { x: px, y: py },
                velocity: Velocity { x: vx, y: vy },
            }
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone)]
struct Area {
    wide: i64,
    tall: i64,
}

fn part1(file_name: &str, area: Area) {
    println!();
    let robots = parse_input(file_name);
    let robots = robots
        .iter()
        .map(|&robot| robot.run_after(100, area.clone()))
        .collect::<Vec<_>>();
    let score = safty_factor(robots, area);
    dbg!(&score);
}

fn safty_factor(robots: Vec<Robot>, area: Area) -> u64 {
    let robots = robots
        .into_iter()
        .filter(|robot| {
            // println!("{:?}, checking", robot);
            let p = robot.position.x != area.wide / 2 && robot.position.y != area.tall / 2;
            if !p {
                // println!("{:?}, in midle", robot);
            }
            p
        })
        .collect::<Vec<Robot>>();

    let mut pp = Vec::new();
    let mut pn = Vec::new();
    let mut np = Vec::new();
    let mut nn = Vec::new();
    robots.iter().for_each(|&robot| {
        match (
            robot.position.x < area.wide / 2,
            robot.position.y < area.tall / 2,
        ) {
            (true, true) => pp.push(robot),
            (true, false) => pn.push(robot),
            (false, true) => np.push(robot),
            (false, false) => nn.push(robot),
        }
    });
    println!("{}, {}, {}, {}", pp.len(), pn.len(), np.len(), nn.len());

    (pp.len() * pn.len() * np.len() * nn.len()) as u64
}

// #[test]
// fn part1_example1() {
//     let area = Area { wide: 11, tall: 7 };
//     part1("input.txt.example1", area);
// }

// #[test]
// fn part1_example() {
//     let area = Area { wide: 11, tall: 7 };
//     part1("input.txt.example", area);
// }

#[test]
fn part1_main() {
    let area = Area {
        wide: 101,
        tall: 103,
    };
    part1("input.txt", area);
}
use crossterm::{
    cursor::{MoveTo, RestorePosition},
    ExecutableCommand,
};
use std::io::{self};

fn print_robots(robots: &[Robot], area: Area, second: u64) -> bool {
    // with function
    io::stdout()
        .execute(MoveTo(1, 0))
        .unwrap()
        .execute(RestorePosition);
    println!("--------\n{} second\n\n", second);

    let mut map: Vec<Vec<char>> = vec![vec![' '; area.wide as usize]; area.tall as usize];
    for robot in robots {
        let Position { x, y } = robot.position;
        let x = x as usize;
        let y = y as usize;
        // map[y][x] = 'O';
        map[y][x] = '█';
    }
    let mut plot = String::new();
    let mut found = false;
    for y in 0..area.tall {
        let mut line = String::new();
        line.push_str(&format!("{:03}", y));
        for x in 0..area.wide {
            line.push(map[y as usize][x as usize]);
        }
        plot.push_str(&line);
        plot.push('\n');
        if line.contains("██████████") {
            println!("------------------------------------- {}", second);
            println!("{}", plot);
            found = true;
        }
    }
    println!("{}", plot);
    if found {
        return true;
    }
    false
}

fn part2_main(file_name: &str, area: Area) {
    println!();
    let mut robots = parse_input(file_name);

    let mut second = 0;
    loop {
        second += 1;
        robots = robots
            .iter()
            .map(|&robot| robot.run_1s(area.clone()))
            .collect::<Vec<_>>();
        if print_robots(&robots, area.clone(), second) {
            println!("second: {second}");
            return;
        }
        // std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

// #[test]
// fn part2_example() {
//     let area = Area { wide: 11, tall: 7 };
//     part2_main("input.txt.example", area);
// }

#[test]
fn part2() {
    let area = Area {
        wide: 101,
        tall: 103,
    };
    part2_main("input.txt", area);
}

fn ifconifg() {
    println!("fjeiwojo")
    
}


fn () {
    
}
