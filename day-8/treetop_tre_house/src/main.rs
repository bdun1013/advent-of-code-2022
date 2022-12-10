use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{BufRead, BufReader},
};

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy, Default)]
struct Point {
    x: u8,
    y: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut map: HashMap<Point, u8> = HashMap::new();
    let mut max_x: u8 = 0;
    let mut max_y: u8 = 0;
    for (y, line_res) in reader.lines().enumerate() {
        let line = line_res?;
        max_y = y as u8;

        line.chars()
            .into_iter()
            .enumerate()
            .map(|(x, c)| (x as u8, c.to_string().parse::<u8>().unwrap()))
            .for_each(|(x, h)| {
                map.insert(Point { x: x, y: y as u8 }, h);
                max_x = x;
            });
    }

    let mut max = 0;
    for (p, h) in map.iter() {
        let x = p.x;
        let y = p.y;

        if x == 0 || x == max_x || y == 0 || y == max_y {
            continue;
        }

        // Left
        let mut left = 0;
        for i in (0..x).rev() {
            let p2 = Point { x: i, y: y };
            let t = map.get(&p2).unwrap();
            if h > t {
                left = left + 1;
            } else if h == t {
                left = left + 1;
                break;
            }
        }

        // Right
        let mut right = 0;
        for i in x + 1..=max_x {
            let p2 = Point { x: i, y: y };
            let t = map.get(&p2).unwrap();
            if h > t {
                right = right + 1;
            } else if h <= t {
                right = right + 1;
                break;
            }
        }

        // Up
        let mut up = 0;
        for i in (0..y).rev() {
            let p2 = Point { x: x, y: i };
            let t = map.get(&p2).unwrap();
            if h > t {
                up = up + 1;
            } else if h == t {
                up = up + 1;
                break;
            }
        }

        // Down
        let mut down = 0;
        for i in y + 1..=max_y {
            let p2 = Point { x: x, y: i };
            let t = map.get(&p2).unwrap();
            if h > t {
                down = down + 1;
            } else if h == t {
                down = down + 1;
                break;
            }
        }

        let score = left * right * up * down;

        if score > max {
            max = score;
        }
    }

    println!("{max}");

    Ok(())
}
