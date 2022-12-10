use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy, Default)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn add(&mut self, motion: Motion) {
        match motion {
            Motion::Up(amount) => {
                self.y = self.y + amount;
            }
            Motion::Right(amount) => {
                self.x = self.x + amount;
            }
            Motion::Down(amount) => {
                self.y = self.y - amount;
            }
            Motion::Left(amount) => {
                self.x = self.x - amount;
            }
        }
    }

    fn is_touching(&self, other: &Point) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        } else if self.x + 1 == other.x && self.y == other.y {
            return true;
        } else if self.x - 1 == other.x && self.y == other.y {
            return true;
        } else if self.x == other.x && self.y + 1 == other.y {
            return true;
        } else if self.x == other.x && self.y - 1 == other.y {
            return true;
        } else if self.x + 1 == other.x && self.y + 1 == other.y {
            return true;
        } else if self.x + 1 == other.x && self.y - 1 == other.y {
            return true;
        } else if self.x - 1 == other.x && self.y + 1 == other.y {
            return true;
        } else if self.x - 1 == other.x && self.y - 1 == other.y {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
enum Motion {
    Up(i16),
    Right(i16),
    Down(i16),
    Left(i16),
}

impl FromStr for Motion {
    type Err = ();
    fn from_str(input: &str) -> Result<Motion, Self::Err> {
        let mut split = input.split(" ");
        let direction = split.next().unwrap();
        let amount = split.next().unwrap().parse::<i16>().unwrap();
        match direction {
            "U" => Ok(Motion::Up(amount)),
            "R" => Ok(Motion::Right(amount)),
            "D" => Ok(Motion::Down(amount)),
            "L" => Ok(Motion::Left(amount)),
            _ => Err(()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut points: HashMap<u8, Cell<Point>> = HashMap::new();
    let mut prev_points: HashMap<u8, Cell<Point>> = HashMap::new();
    let mut visits: HashSet<Point> = HashSet::new();

    for i in 0..10 {
        points.insert(i, Cell::new(Point::default()));
    }

    visits.insert(Point::default());

    for line_res in reader.lines() {
        let line = line_res?;
        let motion = Motion::from_str(&line).unwrap();
        match motion {
            Motion::Up(amount) => {
                for _ in 0..amount {
                    let head_cell = points.get(&0).unwrap();
                    let mut head = head_cell.get();
                    let prev_head = head.clone();
                    prev_points.insert(0, Cell::new(prev_head));
                    head.add(Motion::Up(1));
                    head_cell.set(head);
                    for i in 1..10 {
                        let point_1 = points.get(&(i - 1)).unwrap().get();
                        let point_2_cell = points.get(&i).unwrap();
                        let mut point_2 = point_2_cell.get();
                        if !point_1.is_touching(&point_2) {
                            let previous_point_1 = prev_points.get(&(i - 1)).unwrap().get();
                            prev_points.insert(i, Cell::new(point_2.clone()));
                            point_2 = previous_point_1.clone();
                            point_2_cell.set(point_2);
                            if i == 9 {
                                visits.insert(point_2);
                            }
                        }
                    }
                }
            }
            Motion::Right(amount) => {
                for _ in 0..amount {
                    let head_cell = points.get(&0).unwrap();
                    let mut head = head_cell.get();
                    let prev_head = head.clone();
                    prev_points.insert(0, Cell::new(prev_head));
                    head.add(Motion::Right(1));
                    head_cell.set(head);
                    for i in 1..10 {
                        let point_1 = points.get(&(i - 1)).unwrap().get();
                        let point_2_cell = points.get(&i).unwrap();
                        let mut point_2 = point_2_cell.get();
                        if !point_1.is_touching(&point_2) {
                            let previous_point_1 = prev_points.get(&(i - 1)).unwrap().get();
                            prev_points.insert(i, Cell::new(point_2.clone()));
                            point_2 = previous_point_1.clone();
                            point_2_cell.set(point_2);
                            if i == 9 {
                                visits.insert(point_2);
                            }
                        }
                    }
                }
            }
            Motion::Down(amount) => {
                for _ in 0..amount {
                    let head_cell = points.get(&0).unwrap();
                    let mut head = head_cell.get();
                    let prev_head = head.clone();
                    prev_points.insert(0, Cell::new(prev_head));
                    head.add(Motion::Down(1));
                    head_cell.set(head);
                    for i in 1..10 {
                        let point_1 = points.get(&(i - 1)).unwrap().get();
                        let point_2_cell = points.get(&i).unwrap();
                        let mut point_2 = point_2_cell.get();
                        if !point_1.is_touching(&point_2) {
                            let previous_point_1 = prev_points.get(&(i - 1)).unwrap().get();
                            prev_points.insert(i, Cell::new(point_2.clone()));
                            point_2 = previous_point_1.clone();
                            point_2_cell.set(point_2);
                            if i == 9 {
                                visits.insert(point_2);
                            }
                        }
                    }
                }
            }
            Motion::Left(amount) => {
                for _ in 0..amount {
                    let head_cell = points.get(&0).unwrap();
                    let mut head = head_cell.get();
                    let prev_head = head.clone();
                    prev_points.insert(0, Cell::new(prev_head));
                    head.add(Motion::Left(1));
                    head_cell.set(head);
                    for i in 1..10 {
                        let point_1 = points.get(&(i - 1)).unwrap().get();
                        let point_2_cell = points.get(&i).unwrap();
                        let mut point_2 = point_2_cell.get();
                        if !point_1.is_touching(&point_2) {
                            let previous_point_1 = prev_points.get(&(i - 1)).unwrap().get();
                            prev_points.insert(i, Cell::new(point_2.clone()));
                            point_2 = previous_point_1.clone();
                            point_2_cell.set(point_2);
                            if i == 9 {
                                visits.insert(point_2);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", visits.len());

    Ok(())
}
