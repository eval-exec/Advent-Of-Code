fn parse_input(file_name: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn visit(chars: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    let width = chars[0].len();
    let height = chars.len();
    DIRECTIONS
        .map(|direction| visit_direction(chars, x, y, direction))
        .iter()
        .filter(|&&p| p)
        .count()
}

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

// XMAS
const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn visit_direction(chars: &Vec<Vec<char>>, x: isize, y: isize, direction: (isize, isize)) -> bool {
    let width = chars[0].len();
    let height = chars.len();
    let mut cursor = (x, y);
    for (i, W) in WORD.iter().enumerate() {
        if cursor.0 < 0 || cursor.0 >= height as isize || cursor.1 < 0 || cursor.1 >= width as isize
        {
            return false;
        }

        let char = chars[cursor.0 as usize][cursor.1 as usize];
        if char != *W {
            return false;
        }
        cursor.0 += direction.0;
        cursor.1 += direction.1;
    }
    true
}

fn part1(file_name: &str) {
    let chars = parse_input(file_name);
    let mut sum = 0;
    for (x, _) in chars.iter().enumerate() {
        for (y, _) in chars[0].iter().enumerate() {
            sum += visit(&chars, x as isize, y as isize);
        }
    }
    println!("sum: {}", sum);
}

fn check_mas(chars: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    let width = chars[0].len() as isize;
    let height = chars.len() as isize;
    let cursor = (x, y);

    let top_left_cursor = (cursor.0 - 1, cursor.1 - 1);
    let bottom_right_cursor = (cursor.0 + 1, cursor.1 + 1);
    let top_right_cursor = (cursor.0 - 1, cursor.1 + 1);
    let bottom_left_cursor = (cursor.0 + 1, cursor.1 - 1);

    if [
        top_left_cursor,
        top_right_cursor,
        bottom_left_cursor,
        bottom_right_cursor,
    ]
    .iter()
    .filter(|cursor| cursor.0 < 0 || cursor.1 < 0 || cursor.0 >= height || cursor.1 >= width)
    .count()
        != 0
    {
        return false;
    }
    let mut cs0 = [
        chars[top_left_cursor.0 as usize][top_left_cursor.1 as usize],
        chars[bottom_right_cursor.0 as usize][bottom_right_cursor.1 as usize],
    ];
    cs0.sort();
    let left_right = cs0.eq(&['M', 'S']);

    let mut cs1 = [
        chars[top_right_cursor.0 as usize][top_right_cursor.1 as usize],
        chars[bottom_left_cursor.0 as usize][bottom_left_cursor.1 as usize],
    ];
    cs1.sort();
    let right_right = cs1.eq(&['M', 'S']);

    left_right && right_right
}

fn part2(file_name: &str) {
    let chars = parse_input(file_name);
    let mut sum = 0;
    for (x, _) in chars.iter().enumerate() {
        for (y, _) in chars[0].iter().enumerate() {
            if chars[x][y].eq(&'A') {
                if check_mas(&chars, x as isize, y as isize) {
                    sum += 1;
                }
            }
        }
    }
    println!("part2: {file_name},sum: {}", sum);
}

#[test]
fn test_part_example_1() {
    part1("input.example.1.txt");
}

#[test]
fn test_part_example_2() {
    part1("input.example.2.txt");
}

#[test]
fn test_part1() {
    part1("input.txt");
}

#[test]
fn test_part2_example() {
    part2("input.example.part2.txt");
}

#[test]
fn test_part2() {
    part2("input.txt");
}

fn main() {}
