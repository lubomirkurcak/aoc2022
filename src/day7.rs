use std::{collections::HashMap, io::prelude::*, io::BufReader};

use crate::{Day, Problem};

struct FileTree {
    total_directory_sizes: HashMap<String, usize>,
    immediate_directory_sizes: HashMap<String, usize>,
    immediate_directory_subdirectories: HashMap<String, Vec<String>>,
}
impl FileTree {
    fn new() -> Self {
        Self {
            total_directory_sizes: HashMap::new(),
            immediate_directory_sizes: HashMap::new(),
            immediate_directory_subdirectories: HashMap::new(),
        }
    }

    fn get_directory_size(&mut self, directory: String) -> Option<usize> {
        // TODO(lubo): This made the borrow of 'self' mutable. Investigate if it can be transformed somehow.
        #[allow(clippy::map_entry)]
        if self.total_directory_sizes.contains_key(&directory) {
            self.total_directory_sizes.get(&directory).cloned()
        } else {
            let immediate_size = match self.immediate_directory_sizes.get(&directory) {
                Some(&value) => value,
                None => 0,
            };

            let mut total_size = immediate_size;

            if self
                .immediate_directory_subdirectories
                .contains_key(&directory)
            {
                for subdir in self
                    .immediate_directory_subdirectories
                    .get(&directory)
                    .cloned()
                    .unwrap()
                    .iter()
                {
                    if let Some(size) = self.get_directory_size(subdir.clone()) {
                        total_size += size;
                    }
                }
            }

            self.total_directory_sizes.insert(directory, total_size);

            Some(total_size)
        }
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

impl Problem for Day<7> {
    fn solve_buffer<T>(reader: BufReader<T>) -> Result<(), ()>
    where
        T: std::io::Read,
    {
        let result = 0;

        // let results: Vec<_> = reader.lines().collect();

        let mut tree = FileTree::new();
        let mut current_path = "".to_string();

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(()),
            };

            println!("{}", line);

            let args: Vec<&str> = line.split_whitespace().collect();

            match args[0] {
                "$" => match args[1] {
                    "cd" => match args[2] {
                        "/" => {
                            current_path = "".to_string();
                            println!("{}", current_path);
                        }
                        ".." => {
                            let a = current_path.clone().rfind('/').unwrap();
                            current_path = current_path[..a].to_string();
                            println!("{}", current_path);
                        }
                        dirname => {
                            current_path = format!("{}/{}", current_path, dirname);
                            println!("{}", current_path);
                        }
                    },
                    "ls" => (),
                    _ => panic!(),
                },
                "dir" => {
                    let dirname = args[1];
                    let dirpath = format!("{}/{}", current_path, dirname);
                    tree.immediate_directory_subdirectories
                        .entry(current_path.clone())
                        .or_default()
                        .push(dirpath);
                }
                _ => {
                    let filesize = args[0].parse::<usize>().unwrap();
                    // let filename = args[1];

                    *tree
                        .immediate_directory_sizes
                        .entry(current_path.clone())
                        .or_insert(0) += filesize;
                }
            }
        }

        let sum: usize = tree
            .immediate_directory_sizes
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .iter()
            .map(|dir| {
                if let Some(size) = tree.get_directory_size(dir.clone()) {
                    size
                } else {
                    0
                }
            })
            .sum();

        let sum: usize = tree
            .immediate_directory_sizes
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .iter()
            .map(|dir| {
                if let Some(size) = tree.get_directory_size(dir.clone()) {
                    if size <= 100000 {
                        println!("dir '{}' has size {}", dir, size);
                        return size;
                    }
                }

                0
            })
            .sum();

        println!("{:?}", tree.total_directory_sizes);

        println!("Sum of directory sizes of size at most 100k: {}", sum);

        Ok(())
    }
}
