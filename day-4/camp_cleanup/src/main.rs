use std::{fs::File, error::Error, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    println!("{}", imperative(reader)?);

    Ok(())
}

// fn declarative(reader: BufReader<File>) -> Result<u32, Box<dyn Error>> {
//     reader.lines()
//         .map(Result::unwrap)
//         .map(|line| {
//             let line_split = line.split(",").collect::<Vec<&str>>();
//             let first = line_split.get(0).unwrap();
//             let second = line_split.get(1).unwrap();

//             let first_split = first.split("-").collect::<Vec<&str>>();
//             let second_split = second.split("-").collect::<Vec<&str>>();
//             let first_min = first_split.get(0).unwrap().parse::<u32>().unwrap();
//             let second_min = second_split.get(0).unwrap().parse::<u32>().unwrap();
//             let first_max = first_split.get(1).unwrap().parse::<u32>().unwrap();
//             let second_max = second_split.get(1).unwrap().parse::<u32>().unwrap();

//             (first_min..=first_max, second_min..=second_max)
//         })
//         .filter(|(first_range, second_range)| {


//             let first_min = first_range.min().unwrap();
//             true

//             // (first_range.contains(second_range.min().unwrap()) && first_range.contains(&second_max)) ||
//             // (second_range.contains(&first_min) && second_range.contains(&first_max))
//         });

//     Ok(0)
// }

fn imperative(reader: BufReader<File>) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;
    for line_res in reader.lines() {
        let line = line_res?;
        let line_split = line.split(",").collect::<Vec<&str>>();
        let first = line_split.get(0).unwrap();
        let second = line_split.get(1).unwrap();

        let first_split = first.split("-").collect::<Vec<&str>>();
        let second_split = second.split("-").collect::<Vec<&str>>();
        let first_min = first_split.get(0).unwrap().parse::<u32>()?;
        let second_min = second_split.get(0).unwrap().parse::<u32>()?;
        let first_max = first_split.get(1).unwrap().parse::<u32>()?;
        let second_max = second_split.get(1).unwrap().parse::<u32>()?;

        let first_range = first_min..=first_max;
        let second_range = second_min..=second_max;


        if (first_range.contains(&second_min) || first_range.contains(&second_max)) ||
            (second_range.contains(&first_min) || second_range.contains(&first_max)) {
            sum = sum + 1;
            println!("YES {}-{},{}-{}", first_min, first_max, second_min, second_max);
        } else {
            println!("NO {}-{},{}-{}", first_min, first_max, second_min, second_max);
        }

    }
    return Ok(sum);
} 