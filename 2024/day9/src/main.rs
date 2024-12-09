fn main() {
    println!("Hello, world!");
}

fn parse_input(file_name: &str) -> Vec<u32> {
    let line = std::fs::read_to_string(&file_name).unwrap();
    line.trim()
        .chars()
        .map(|c| {
            let v = c.to_digit(10).unwrap();
            v
        })
        .collect()
}

fn part1(file_name: &str) -> usize {
    println!("");
    let input = parse_input(file_name);

    let mut represent: Vec<Option<usize>> = input
        .chunks(2)
        .enumerate()
        .map(|(file_id, pair)| {
            // println!("file_id: {file_id}");
            let mut results: Vec<Option<usize>> = Vec::new();
            let mut file_count = pair[0];
            while file_count > 0 {
                results.push(Some(file_id));
                // results.extend(file_ids.clone());
                file_count -= 1;
            }

            if let Some(space_count) = pair.get(1) {
                let mut space_count = space_count.to_owned();

                while space_count > 0 {
                    results.push(None);
                    space_count -= 1;
                }
            }
            results
        })
        .flatten()
        .collect();

    let mut empty_indexs = Vec::new();
    for i in 0..represent.len() {
        if represent[i] == None {
            empty_indexs.push(i);
        }
    }
    println!("{:?}", represent);

    {
        for empty_index in empty_indexs {
            let mut digit_last = None;
            while digit_last == None {
                digit_last = represent.pop().unwrap();
            }

            if empty_index >= represent.len() {
                represent.push(digit_last);
                break;
            }
            represent[empty_index] = digit_last;
        }
    }

    println!("{:?}", represent);
    let checksum = represent
        .iter()
        .enumerate()
        .map(|(index, d)| index * d.unwrap())
        .sum();

    checksum
}

#[test]
fn part1_example() {
    let checksum = part1("input.txt.example");
    println!("checksum: {checksum}");
}

#[test]
fn part1_example1() {
    let checksum = part1("input.txt.example1");
    println!("checksum: {checksum}");
}

#[test]
fn part1_main() {
    let checksum = part1("input.txt");
    println!("checksum: {checksum}");
}
