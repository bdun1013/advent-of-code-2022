use std::{fs::File, io::{BufReader, self, BufRead}};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut max_calories = (0, 0, 0);
    let mut current_calories = 0;
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        if !line.is_empty() {
            let calories = line.parse::<i32>().unwrap();
            current_calories = current_calories + calories;
        } else {
            if current_calories > max_calories.0 {
                max_calories.2 = max_calories.1;
                max_calories.1 = max_calories.0;
                max_calories.0 = current_calories;
            } else if current_calories > max_calories.1 {
                max_calories.2 = max_calories.1;
                max_calories.1 = current_calories;
            } else if current_calories > max_calories.2 {
                max_calories.2 = current_calories;
            }
            
            current_calories = 0;
        }
    }

    println!("{} {} {}", max_calories.0, max_calories.1, max_calories.2);

    println!("{}", max_calories.0 + max_calories.1 + max_calories.2);

    Ok(())
}
