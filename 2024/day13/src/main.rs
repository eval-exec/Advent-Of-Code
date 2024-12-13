use std::cmp;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Button {
    X: u64,
    Y: u64,
}

#[derive(Debug, Clone, Copy)]
struct Prize {
    X: u64,
    Y: u64,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn parse_button(line: &str) -> Option<Button> {
    let parts: Vec<&str> = line.split(&['+', ','][..]).collect();
    let x_offset = parts.get(1)?.parse().ok()?;
    let y_offset = parts.get(3)?.parse().ok()?;
    Some(Button {
        X: x_offset,
        Y: y_offset,
    })
}

fn parse_prize(line: &str) -> Option<Prize> {
    let parts: Vec<&str> = line.split(&['=', ','][..]).collect();
    let x = parts.get(1)?.parse().ok()?;
    let y = parts.get(3)?.parse().ok()?;
    Some(Prize { X: x, Y: y })
}

fn parse_input(file_name: &str) -> Vec<Machine> {
    //
    let content = std::fs::read_to_string(file_name).unwrap();
    let cases = content.split("\n\n").collect::<Vec<_>>();
    cases
        .iter()
        .map(|case| {
            let mut lines = case.lines();
            let button_a_line = lines.next().unwrap();
            let button_b_line = lines.next().unwrap();
            let prize_line = lines.next().unwrap();

            let button_a = parse_button(&button_a_line).unwrap();
            let button_b = parse_button(&button_b_line).unwrap();
            let prize = parse_prize(&prize_line).unwrap();
            let machine = Machine {
                button_a,
                button_b,
                prize,
            };
            machine
        })
        .collect::<Vec<_>>()
}

fn part1(file_name: &str) {
    println!();
    let machines = parse_input(file_name);
    let feawest_tokens_sum: u64 = machines
        .iter()
        .map(|machine| {
            let fe = machine.fewest_tokens();
            fe
        })
        .filter(|&sum| sum != u64::MAX)
        .sum();
    println!("{}, fewest tokens: {}", file_name, feawest_tokens_sum);
}

impl Machine {
    fn part2_fix_prize(self) -> Self {
        let Machine {
            button_a,
            button_b,
            prize,
        } = self;
        Machine {
            button_a,
            button_b,
            prize: Prize {
                X: prize.X + 10000000000000,
                Y: prize.Y + 10000000000000,
            },
        }
    }
    fn fewest_tokens(&self) -> u64 {
        let mut count_a = 0;
        let mut fewest_tokens = u64::MAX;
        loop {
            if self.prize.X < count_a * self.button_a.X {
                return fewest_tokens;
            }
            if self.prize.Y < count_a * self.button_a.Y {
                return fewest_tokens;
            }
            let remain_b_X = self.prize.X - count_a * self.button_a.X;
            let remain_b_Y = self.prize.Y - count_a * self.button_a.Y;
            if remain_b_X % self.button_b.X != 0 || remain_b_Y % self.button_b.Y != 0 {
                count_a += 1;
                continue;
            }

            let count_b = remain_b_X / self.button_b.X;
            if remain_b_X / self.button_b.X != remain_b_Y / self.button_b.Y {
                count_a += 1;
                continue;
            }

            fewest_tokens = cmp::min(fewest_tokens, count_a * 3 + count_b);
            count_a += 1;
        }
        fewest_tokens
    }
    fn fewest_tokens_algo(&self) -> u64 {
        let mut count_a = 0;
        let mut fewest_tokens = u64::MAX;
        if let Some(solution) = solve(&self.button_a, &self.button_b, &self.prize) {
            fewest_tokens = cmp::min(fewest_tokens, solution.count_a * 3 + solution.count_b);
        }
        fewest_tokens
    }
}

struct Solution {
    count_a: u64,
    count_b: u64,
}

fn solve(button_a: &Button, button_b: &Button, prize: &Prize) -> Option<Solution> {
    let det = button_a.X as i64 * button_b.Y as i64 - button_a.Y as i64 * button_b.X as i64;
    if det == 0 {
        return None;
    }

    let count_a_val = (prize.X as i64 * button_b.Y as i64 - prize.Y as i64 * button_b.X as i64);
    let count_b_val = (prize.Y as i64 * button_a.X as i64 - prize.X as i64 * button_a.Y as i64);

    if count_a_val % det == 0 && count_b_val % det == 0 {
        let count_a = count_a_val / det;
        let count_b = count_b_val / det;
        if count_a >= 0 && count_b >= 0 {
            return Some(Solution {
                count_a: count_a as u64,
                count_b: count_b as u64,
            });
        }
    }

    None
}

#[test]
fn part1_example() {
    part1("input.txt.example");
}

// #[test]
// fn part1_main() {
//     part1("input.txt");
// }

fn part2(file_name: &str) {
    println!();
    let machines = parse_input(file_name);
    let part2_machines = machines
        .iter()
        .map(|machine| machine.part2_fix_prize())
        .collect::<Vec<_>>();
    let feawest_tokens_sum: u64 = part2_machines
        .iter()
        .map(|machine| {
            let fe = machine.fewest_tokens_algo();
            fe
        })
        .filter(|&sum| sum != u64::MAX)
        .sum();
    println!("{}, fewest tokens: {}", file_name, feawest_tokens_sum);
}

#[test]
fn part2_example() {
    part2("input.txt.example");
}

#[test]
fn part2_main() {
    part2("input.txt");
}
