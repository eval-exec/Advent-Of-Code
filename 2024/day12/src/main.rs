use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

fn main() {
    println!("Hello, world!");
}

fn parse_input(file_name: &str) -> Vec<Vec<char>> {
    let contents = std::fs::read_to_string(file_name).unwrap();
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Default)]
struct Cursor {
    up: Option<char>,
    down: Option<char>,
    left: Option<char>,
    right: Option<char>,
}

#[derive(Default)]
struct PlantArea {
    plant: char,
    perimeter: u64,
    area: u64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    i: usize,
    j: usize,
}
fn visit(garden: &Vec<Vec<char>>) -> u64 {
    let mut point_plant_areas = HashMap::new();
    let mut plant_areas = Vec::new();

    for i in 0..garden.len() {
        for j in 0..garden[0].len() {
            let point = Point { i, j };
            visit_inner(point, garden, &mut point_plant_areas, &mut plant_areas);
        }
    }

    plant_areas
        .iter()
        .map(|plant_area| {
            let c = plant_area.borrow().plant;
            let area = plant_area.borrow().area;
            let perimeter = plant_area.borrow().perimeter;
            println!("{c}: {area}*{perimeter}");
            area * perimeter
        })
        .sum()
}

fn visit_inner(
    point: Point,
    garden: &Vec<Vec<char>>,
    point_plant_areas: &mut HashMap<Point, Rc<RefCell<PlantArea>>>,
    plant_areas: &mut Vec<Rc<RefCell<PlantArea>>>,
) {
    let plant = garden[point.i][point.j];
    let neighbors = neighbors(point, garden);
    neighbors.iter().for_each(|&neighbor| {
        //

        let plant_area: Rc<RefCell<PlantArea>> = match point_plant_areas.entry(neighbor) {
            Entry::Occupied(entry) => {
                let existing_area = entry.get().clone();

                if existing_area.get_mut().plant.eq(&plant) {
                    existing_area
                } else {
                    let new_plant_area = Rc::new(RefCell::new(PlantArea {
                        plant,
                        ..Default::default()
                    }));
                    plant_areas.push(new_plant_area.clone());
                    point_plant_areas.insert(point, new_plant_area.clone());

                    new_plant_area
                }
            }
            Entry::Vacant(v) => {
                let plant_area = Rc::new(RefCell::new(PlantArea {
                    plant,
                    ..Default::default()
                }));
                plant_areas.push(plant_area.clone());
                v.insert(plant_area.clone());

                plant_area
            }
        };

        let mut plant_area = plant_area.borrow_mut();
        plant_area.area += 1;

        let mut cursor = Cursor::default();
        set_cursor(&mut cursor, point, garden);

        if cursor.left.map_or(true, |left| left != plant) {
            plant_area.perimeter += 1;
        }
        if cursor.right.map_or(true, |right| right != plant) {
            plant_area.perimeter += 1;
        }
        if cursor.up.map_or(true, |up| up != plant) {
            plant_area.perimeter += 1;
        }
        if cursor.down.map_or(true, |up| up != plant) {
            plant_area.perimeter += 1;
        }
        visit_inner(neighbor, garden, point_plant_areas, plant_areas);
    });
}

fn neighbors(point: Point, garden: &Vec<Vec<char>>) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let Point { i, j } = point;
    if point.i != 0 {
        neighbors.push(Point { i: i - 1, j });
    }
    if point.j != 0 {
        neighbors.push(Point { i, j: j - 1 });
    }
    if point.i != garden.len() - 1 {
        neighbors.push(Point { i: i + 1, j })
    }
    if point.j != garden[0].len() - 1 {
        neighbors.push(Point { i, j: j + 1 })
    }
    neighbors
}

fn part1(garden: Vec<Vec<char>>) -> u64 {
    println!();
    visit(&garden)
}

fn set_cursor(cursor: &mut Cursor, point: Point, garden: &Vec<Vec<char>>) {
    //
    let Point { i, j } = point;
    *cursor = Cursor::default();
    if i != 0 {
        cursor.up = Some(garden[i - 1][j]);
    }
    if j != 0 {
        cursor.left = Some(garden[i][j - 1]);
    }
    if i != garden.len() - 1 {
        cursor.down = Some(garden[i + 1][j]);
    }
    if j != garden[0].len() - 1 {
        cursor.right = Some(garden[i][j + 1]);
    }
}

// #[test]
// fn plant_example1() {
//     let garden = parse_input("input.txt.example1");
//     let price = part1(garden);
//     println!("price example: {price}")
// }

// #[test]
// fn plant_exampleo() {
//     let garden = parse_input("input.txt.exampleo");
//     let price = part1(garden);
//     println!("price example: {price}")
// }

#[test]
fn plant_example() {
    let garden = parse_input("input.txt.example");
    let price = part1(garden);
    println!("price example: {price}")
}

// #[test]
// fn plant_main() {
//     let garden = parse_input("input.txt");
//     let price = part1(garden);
//     println!("price: {price}")
// }
