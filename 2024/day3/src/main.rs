fn parse_input() -> Vec<String> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    input.lines().map(|line| line.to_string()).collect()
}

// fn check(line: &str, index: usize) -> (u64, usize) {
//     let i = line[index..].find("mul(");
//     if i.is_none() {
//         return (0, line.len());
//     }
//     let mut left = String::new();
//     let mut right = String::new();

//     let mut meet_star = false;
//     let mut success = false;
//     let mut j = index + i.unwrap() + 4;
//     while j < line.len() {
//         let j_char = line.chars().nth(j).unwrap();
//         if !meet_star {
//             if j_char == ',' {
//                 meet_star = true;
//             } else if j_char >= '0' && j_char <= '9' {
//                 left.push(j_char);
//             } else {
//                 return (0, j);
//             }
//         } else {
//             if j_char >= '0' && j_char <= '9' {
//                 right.push(j_char);
//             } else if j_char == ')' {
//                 success = true;
//                 break;
//             } else {
//                 return (0, j);
//             }
//         }
//         j += 1;
//     }

//     if success {
//         (
//             left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap(),
//             j,
//         )
//     } else {
//         (0, j)
//     }
// }

// fn part_1() {
//     let input = parse_input();
//     let sum: u64 = input
//         .iter()
//         .map(|line| {
//             let mut index = 0;
//             let mut sum = 0;
//             while index < line.len() {
//                 let (result, next) = check(&line, index);
//                 sum += result;
//                 index = next;
//             }
//             sum
//         })
//         .sum();
//     println!("sum: {sum}")
// }

// fn check2(&mut enable: bool, line: &str, index: usize) -> (u64, usize) {
//     let i = line[index..].find("mul(");
//     if i.is_none() {
//         return (0, line.len());
//     }
//     let mut left = String::new();
//     let mut right = String::new();

//     let mut meet_star = false;
//     let mut success = false;
//     let mut j = index + i.unwrap() + 4;
//     while j < line.len() {
//         let j_char = line.chars().nth(j).unwrap();
//         //do()
//         if j + 4 < line.len() {
//             println!("{}", line[j..j + 4].to_string());
//             if line[j..j + 4].eq("do()") {
//                 println!("enablne");
//                 enable = true;
//                 return (0, j + 4);
//             }
//         }
//         // don't()
//         if j + 6 < line.len() {
//             if line[j..j + 7].eq("don't") {
//                 enable = false;
//                 return (0, j + 6);
//             }
//         }

//         if enable {
//             if !meet_star {
//                 if j_char == ',' {
//                     meet_star = true;
//                 } else if j_char >= '0' && j_char <= '9' {
//                     left.push(j_char);
//                 } else {
//                     return (0, j);
//                 }
//             } else {
//                 if j_char >= '0' && j_char <= '9' {
//                     right.push(j_char);
//                 } else if j_char == ')' {
//                     success = true;
//                     break;
//                 } else {
//                     return (0, j);
//                 }
//             }
//         }
//         j += 1;
//     }

//     // println!("{left},{meet_star}, {right}");
//     if success {
//         (
//             left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap(),
//             j,
//         )
//     } else {
//         (0, j)
//     }
// }

// fn check_mul(line: &str, index: usize) -> u64 {
//     let mut meet_star = false;
//     let mut left = String::new();
//     let mut right = String::new();
//     let j_char = line.chars().nth(index).unwrap();
//     let mut j = index;
//     let mut success = false;
//     while j < line.len() {
//         if !meet_star {
//             if j_char == ',' {
//                 meet_star = true;
//             } else if j_char >= '0' && j_char <= '9' {
//                 left.push(j_char);
//             } else {
//                 return 0;
//             }
//         } else {
//             if j_char >= '0' && j_char <= '9' {
//                 right.push(j_char);
//             } else if j_char == ')' {
//                 success = true;
//                 break;
//             } else {
//                 return 0;
//             }
//         }
//     }

//     if success {
//             left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap(),
//     } else {
//         0
//     }
// }

// fn check3(mut enable: bool, line: &str, index: uszie) -> (u64, usize) {
//     let mut j = index;
//     while j < line.len() {
//         if j + 4 < line.len() {
//             println!("{}", line[j..j + 4].to_string());
//             if line[j..j + 4].eq("do()") {
//                 println!("enablne");
//                 enable = true;
//                 return (0, j + 4);
//             }
//         }
//         if j + 6 < line.len() {
//             println!("{}", line[j..j + 4].to_string());
//             if line[j..j + 7].eq("don't") {
//                 println!("disable");
//                 enable = true;
//                 return (0, j + 4);
//             }
//         }

//         if j + 4 < line.len() {
//             println!("{}", line[j..j + 4].to_string());
//             if line[j..j + 4].eq("mul(") {
//                 println!("mutl(");
//             }
//         }
//         let mut meet_star = false;
//         j += 4;
//         let j_char = line.chars().nth(j).unwrap();

//         if !meet_star {
//             if j_char == ',' {
//                 meet_star = true;
//             } else if j_char >= '0' && j_char <= '9' {
//                 left.push(j_char);
//             } else {
//                 return (0, j);
//             }
//         } else {
//             if j_char >= '0' && j_char <= '9' {
//                 right.push(j_char);
//             } else if j_char == ')' {
//                 success = true;
//                 break;
//             } else {
//                 return (0, j);
//             }
//         }
//     }
// }

// fn part_2() {
//     let input = parse_input();
//     let mut enable = true;
//     let sum: u64 = input
//         .iter()
//         .map(|line| {
//             let mut index = 0;
//             let mut sum = 0;
//             while index < line.len() {
//                 let (result, next) = check2(&mut enable, &line, index);
//                 sum += result;
//                 index = next;
//             }
//             sum
//         })
//         .sum();
//     println!("sum: {sum}")
// }

fn chk_par(line: &str, index: usize) -> Option<(usize, usize)> {
    let left = line[index..].find("(");
    let right = line[index + 1..].find(")");
    if left.is_none() || right.is_none() {
        return None;
    }
    let left = index + left.unwrap();
    let right = index + 1 + right.unwrap();
    if left >= right {
        return chk_par(line, left);
    }
    if line[left + 1..right]
        .chars()
        .filter(|&c| {
            if (c >= '0' && c <= '9') || c == ',' {
                false
            } else {
                // println!("{}", line[left + 1..right].to_string());
                true
            }
        })
        .count()
        > 0
    {
        return chk_par(line, index + 1);
    }
    Some((left, right))
}

enum Head {
    Do,    // do()
    Donot, //don't()
    Mul,   //mul()
    Other, //...
}

fn check_before(line: &str, index: usize) -> Head {
    // println!("check before:{}", line[0..index].to_string());
    if index >= 2 && line[index - 2..index].eq("do") {
        return Head::Do;
    }
    if index >= 3 && line[index - 3..index].eq("mul") {
        return Head::Mul;
    }
    if index >= 5 && line[index - 5..index].eq("don't") {
        return Head::Donot;
    }

    Head::Other
}

fn chk(line: &str, start: usize, end: usize) -> u64 {
    let star = line[start..end].find(",");
    if star.is_none() {
        return 0;
    }
    let star = start + star.unwrap();
    let left_str = &line[start + 1..star];
    let right_str = &line[star + 1..end];
    let left: u64 = left_str.parse().unwrap_or_default();
    let right: u64 = right_str.parse().unwrap_or_default();
    println!("{left_str}*{right_str}={left}*{right}");
    left * right
}

fn part_2() {
    let input = parse_input();
    let mut enable = true;
    let total: u64 = input
        .iter()
        .map(|line| {
            let mut result = 0;
            let mut index = 0;
            while index < line.len() {
                if let Some((left, right)) = chk_par(line, index) {
                    match check_before(line, left) {
                        Head::Do => {
                            if left + 1 == right {
                                println!("do");
                                enable = true;
                            }
                        }
                        Head::Donot => {
                            if left + 1 == right {
                                println!("don't");
                                enable = false
                            }
                        }
                        Head::Mul => {
                            if enable {
                                result += chk(line, left, right);
                            }
                        }
                        Head::Other => {}
                    }
                    index = right + 1;
                } else {
                    break;
                }
            }
            result
        })
        .sum();
    println!("total: {}", total);
}

fn main() {
    // part_1();
    part_2();
}
