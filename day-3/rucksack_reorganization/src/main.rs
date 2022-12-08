use std::{fs::File, io::{BufReader, BufRead}, error::Error, collections::HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    let mut group: Vec<HashSet<char>> = vec![];

    for line_res in reader.lines() {
        let line = line_res?;

        group.push(line.chars().clone().collect());

        if group.len() % 3 == 0 {
            let one = group.get(0).unwrap();
            let two = group.get(1).unwrap();
            let three: HashSet<&char> = group.get(2).unwrap().iter().collect();

            let one_two_intersection: Vec<&char> = one.intersection(two).collect();

            let one_two_intersection_set: HashSet<&char> = one_two_intersection.into_iter().collect();

            let intersection: Vec<&&char> = three.intersection(&one_two_intersection_set).collect();

            println!("intersection: {:?}", intersection);

            let rucksack_intersection_sum: u32 = intersection.iter()
                .map(|c| -> u32 {
                    if c.is_ascii_lowercase() {
                        (***c as u32) - 96
                    } else {
                        (***c as u32) - 64 + 26
                    }
                    
                })
                .sum();

            sum = sum + rucksack_intersection_sum;

            group.clear();
        }

    }

    println!("{}", sum);

    return Ok(())
}
