use std::{fs, collections::HashSet};

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let len = content.len();

    for i in 0..len-3 {
        let slice = &content[i..i+14];
        let chars: HashSet<char> = HashSet::from_iter(slice.chars().into_iter());
        if chars.len() == 14 {
            println!("{}", i + 14);
            break;
        }
    }
}
