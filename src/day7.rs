use std::{collections::HashMap, fmt::Display, io::prelude::*, io::BufReader};

use crate::{Day, Problem};

struct FileTree {
    immediate_directory_subdirectories: HashMap<String, Vec<String>>,
    immediate_directory_files: HashMap<String, Vec<String>>,
    file_sizes: HashMap<String, usize>,
}

trait PathFormatter {
    fn format(
        f: &mut std::fmt::Formatter<'_>,
        path: &str,
        filetype: &str,
        size: usize,
    ) -> std::fmt::Result;
}

struct LightFormatter;
impl PathFormatter for LightFormatter {
    fn format(
        f: &mut std::fmt::Formatter<'_>,
        path: &str,
        filetype: &str,
        size: usize,
    ) -> std::fmt::Result {
        let count = path.match_indices('/').count();
        let name = match count {
            0 => "/",
            _ => &path[path.rfind('/').unwrap() + 1..],
        };
        writeln!(
            f,
            "{:width$}- {} ({}, size = {})",
            "",
            name,
            filetype,
            size,
            width = count * 2
        )
    }
}

struct FullFormatter;
impl PathFormatter for FullFormatter {
    fn format(
        f: &mut std::fmt::Formatter<'_>,
        path: &str,
        filetype: &str,
        size: usize,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} ({})",
            // filetype.chars().next().unwrap_or(' '),
            match filetype {
                "dir" => 'd',
                _ => ' ',
            },
            path,
            size
        )
    }
}

impl Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_display::<LightFormatter>(f)
        // self.format_display::<FullFormatter>(f)
    }
}

// NOTE(lubo): Simple sketch to cache file tree sizes. Basically a cache decorator.
// Similarly to previous tasks, not really needed to solve the problem as it is already finishing pretty much instantly.
#[allow(dead_code)]
struct CachedFileTree {
    tree: FileTree,
    dir_size_cache: HashMap<String, usize>,
}

impl FileTree {
    fn new() -> Self {
        Self {
            immediate_directory_subdirectories: HashMap::new(),
            immediate_directory_files: HashMap::new(),
            file_sizes: HashMap::new(),
        }
    }

    fn format_display<T: PathFormatter>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut dirs = self
            .immediate_directory_subdirectories
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        dirs.sort();
        for dir in dirs.iter() {
            let size = self.get_directory_size(dir).unwrap();
            T::format(f, dir, "dir", size)?;

            let files = self.immediate_directory_files.get(dir);
            if let Some(files) = files {
                for file in files.iter() {
                    let size = *self.file_sizes.get(file).unwrap();
                    T::format(f, file, "file", size)?;
                }
            }
        }

        write!(f, "")
    }

    fn get_local_files_size(&self, directory: &str) -> Option<usize> {
        self.immediate_directory_files.get(directory).map(|files| {
            files
                .iter()
                .map(|file| self.file_sizes.get(file).unwrap())
                .sum::<usize>()
        })
    }

    fn get_directory_size(&self, directory: &str) -> Option<usize> {
        if self
            .immediate_directory_subdirectories
            .contains_key(directory)
        {
            let mut total_size = 0;

            if let Some(local_files_size) = self.get_local_files_size(directory) {
                total_size += local_files_size;
            }

            if let Some(subdirs) = self.immediate_directory_subdirectories.get(directory) {
                total_size += subdirs
                    .iter()
                    .map(|x| self.get_directory_size(x).unwrap_or(0))
                    .sum::<usize>();
            }

            Some(total_size)
        } else {
            None
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
        let mut tree = FileTree::new();
        let mut current_path = "".to_string();

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => panic!(),
            };

            let args: Vec<&str> = line.split_whitespace().collect();

            match args[0] {
                "$" => match args[1] {
                    "cd" => match args[2] {
                        "/" => {
                            current_path = "".to_string();
                        }
                        ".." => {
                            let a = current_path.clone().rfind('/').unwrap();
                            current_path = current_path[..a].to_string();
                        }
                        dirname => {
                            let new_path = format!("{}/{}", current_path, dirname);
                            assert!(tree
                                .immediate_directory_subdirectories
                                .get(&current_path)
                                .unwrap()
                                .contains(&new_path));
                            current_path = new_path;
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
                        .push(dirpath.clone());
                    tree.immediate_directory_subdirectories
                        .entry(dirpath.clone())
                        .or_default();
                }
                _ => {
                    let filesize = args[0].parse::<usize>().unwrap();
                    let filename = args[1];
                    let filepath = format!("{}/{}", current_path, filename);

                    tree.immediate_directory_files
                        .entry(current_path.clone())
                        .or_default()
                        .push(filepath.clone());

                    tree.file_sizes.insert(filepath, filesize);
                }
            }
        }

        // println!("{}", tree);

        let sum: usize = tree
            .immediate_directory_subdirectories
            .keys()
            .into_iter()
            .map(|dir| tree.get_directory_size(dir).unwrap_or(0))
            .filter(|&x| x <= 100000)
            .sum();
        println!("Sum of directory sizes of size at most 100k: {}", sum);

        let used_memory = tree.get_directory_size("").unwrap();
        let total_memory = 70000000;
        let memory_needed = 30000000;
        let memory_available = total_memory - used_memory;
        let need_to_free = memory_needed - memory_available;
        let smallest_such_dir = tree
            .immediate_directory_subdirectories
            .keys()
            .into_iter()
            .map(|dir| tree.get_directory_size(dir).unwrap_or(0))
            .filter(|&x| x >= need_to_free)
            .min()
            .unwrap();
        println!(
            "Size of smallest dir to free enough space: {}",
            smallest_such_dir
        );

        Ok(())
    }
}
