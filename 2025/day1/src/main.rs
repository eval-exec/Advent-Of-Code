fn main() {
    let content = std::fs::read_to_string("input.txt").expect("failed to read input.txt");
    let instructs = parse(&content);
    let r = solve(instructs);
    println!("{r}")
}

fn parse(content: &str) -> Vec<Instruct> {
    let mut res = Vec::new();
    for line in content.lines() {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let dir = match dir {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        };
        let steps = chars.as_str().parse::<u32>().unwrap();
        res.push(Instruct { dir, steps });
    }
    res
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruct {
    dir: Direction,
    steps: u32,
}

fn solve(instructions: Vec<Instruct>) -> u32 {
    let mut dial = 0;
    let mut position: i32 = 50;
    for instruction in instructions {
        dbg!(&instruction);
        let Instruct { dir, steps } = instruction;

        match dir {
            Direction::Left => {
                //
                position -= steps as i32
            }
            Direction::Right => {
                //
                position += steps as i32
            }
        }
        position %= 100;
        if position < 0 {
            position += 100;
        }
        if position == 0 {
            dial += 1;
        }
        dbg!(&position);
    }
    dial
}
