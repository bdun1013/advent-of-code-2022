use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;
    for line_res in reader.lines() {
        let line = line_res?;
        if line == "noop" {
            cycle = cycle + 1;
            // During Cycle 1
            during_cycle(x, cycle, &mut sum);
        } else {
            cycle = cycle + 1;
            // During Cycle 2
            during_cycle(x, cycle, &mut sum);
            cycle = cycle + 1;
            // During Cycle 3
            during_cycle(x, cycle, &mut sum);
            let add_value = line
                .split(" ")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            x = x + add_value;
        }
    }

    println!();
    println!("{}", sum);

    Ok(())
}

fn during_cycle(x: i32, cycle: i32, sum: &mut i32) {
    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        *sum = *sum + (x * cycle);
    }

    let position = (cycle % 40) - 1;

    if position % 40 == 0 {
        println!();
    }

    if x == position || x == position + 1 || x == position - 1 {
        print!("#");
    } else {
        print!(".");
    }
}
