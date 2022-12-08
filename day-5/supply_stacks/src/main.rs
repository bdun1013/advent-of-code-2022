use std::{error::Error, io::{BufReader, BufRead}, fs::File, collections::VecDeque};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut empty_line_parsed = false;
    let mut stacks: [VecDeque<String>; 9] = Default::default();
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line_res in reader.lines() {
        let line = line_res?;
        if line == "" {
            empty_line_parsed = true;
            continue;
        }

        if !empty_line_parsed {
            if line.contains("[") && line.contains("]") {
                for i in 0..9 {
                    let c = line.chars().nth(1 + (4 * i)).unwrap();
                    if c != ' ' {
                        let stack = &mut stacks[i];
                        stack.push_back(c.to_string());
                    }
                }
            } else {
                // println!("cargo index line");
            }
        } else {
            for cap in re.captures_iter(&line) {
                let amount = &cap[1].parse::<i32>().unwrap();
                let src = &cap[2].parse::<usize>().unwrap() - 1;
                
                let dest = &cap[3].parse::<usize>().unwrap() - 1;

                let mut temp = vec![];
                
                for _ in 0..*amount {
                    let src_stack = &mut stacks[src];
                    let popped_value = src_stack.pop_front().unwrap();
                    temp.push(popped_value);
                }

                temp.into_iter()
                    .rev()
                    .for_each(|item| {
                        let dest_stack = &mut stacks[dest];
                        dest_stack.push_front(item);
                    });

                // for _ in 0..*amount {
                //     let dest_stack = &mut stacks[dest];
                //     dest_stack.
                //     dest_stack.push_front(popped_value);
                // }
                println!("move {} from {} to {}", amount, src, dest);
            }
        }
    }

    for i in 0..9 {
        let stack = &stacks[i];
        println!("stack {} {:?}", i + 1, stack);
    }

    Ok(())
}