use std::{collections::{HashMap, HashSet}, path::{PathBuf}, io::{BufReader, BufRead}, fs, error::Error, str::FromStr};

#[derive(Debug)]
struct Directory {
    path: String,
    files: HashSet<File>,
}

impl Directory {
    fn size(&self) -> u32 {
        self.files.iter()
            .map(|f| f.size)
            .sum()
    }
}

impl Default for Directory {
    fn default() -> Self {
        Directory {
            path: String::new(),
            files: HashSet::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
enum Command {
    Ls(),
    Cd(String),
}

impl FromStr for Command {

    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        if input.starts_with("ls") {
            return Ok(Command::Ls());
        } else if input.starts_with("cd") {
            return Ok(Command::Cd(String::from(&input[3..])));
        } else {
            return Err(());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut filesystem: HashMap<String, Directory> = HashMap::new();
    let mut path_buf = PathBuf::from("/");
    for line_res in reader.lines().skip(1) {
        let line = line_res?;
        if line.starts_with("$") {
            let command = Command::from_str(&line[2..]).unwrap();
            match command {
                Command::Ls() => {
                }
                Command::Cd(path) => {
                    if path == ".." {
                        path_buf.pop();
                    } else {
                        path_buf.push(path);
                    }
                    let dir = filesystem.entry(path_buf.to_str().unwrap().to_owned()).or_insert(Directory::default());
                    dir.path = path_buf.to_str().unwrap().to_owned();
                }
            }
        } else {
            if line.starts_with("dir") {
                let new_path = path_buf.join(&line[4..]);
                let dir = filesystem.entry(path_buf.to_str().unwrap().to_owned()).or_insert(Directory::default());
                dir.path = new_path.to_str().unwrap().to_owned();
            } else {
                let split = line.split(" ").collect::<Vec<&str>>();
                let size = split.get(0).unwrap().parse::<u32>().unwrap();
                let name = split.get(1).unwrap().to_owned().to_owned();
                let dir = filesystem.entry(path_buf.to_str().unwrap().to_owned()).or_insert(Directory::default());
                dir.path = path_buf.to_str().unwrap().to_owned();
                dir.files.insert(File {
                    name: name,
                    size: size,
                });

            }
        }
    }

    let mut sum: u32 = 0;

    filesystem.keys().for_each(|path| {
        let dir = filesystem.get(path).unwrap();
        let dir_file_sizes = dir.size();

        let sub_dir_sizes: u32 = filesystem.keys()
            .filter(|p| p.starts_with(path) && **p != *path)
            .map(|p| filesystem.get(p).unwrap())
            .map(|d| d.size())
            .sum();

        let size = dir_file_sizes + sub_dir_sizes;

        if size <= 100000 {
            sum = sum + size;
        }

    });
    
    println!("{}", sum);

    let total_disk_space = 70000000;
    let needed_free_space = 30000000;
    let used_space: u32 = filesystem.keys()
        .map(|p| filesystem.get(p).unwrap())
        .map(|d| d.size())
        .sum();
    let unused_space = total_disk_space - used_space;

    let space_to_free = needed_free_space - unused_space;

    let mut min_dir_size_to_remove = u32::MAX;
    filesystem.keys().for_each(|path| {
        let dir = filesystem.get(path).unwrap();
        let dir_file_sizes = dir.size();

        let sub_dir_sizes: u32 = filesystem.keys()
            .filter(|p| p.starts_with(path) && **p != *path)
            .map(|p| filesystem.get(p).unwrap())
            .map(|d| d.size())
            .sum();

        let size = dir_file_sizes + sub_dir_sizes;

        if size >= space_to_free {
            if size < min_dir_size_to_remove {
                min_dir_size_to_remove = size;
            }
        }

    });

    println!("{}", min_dir_size_to_remove);



    Ok(())
}
