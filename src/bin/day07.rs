use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day07.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    const MAX_SIZE: i32 = 100000;
    input
        .lines()
        .fold(
            (0, Vec::new(), Vec::new()),
            |(mut result, mut path_stack, mut size_stack), line| {
                let split: Vec<&str> = line.split_whitespace().collect();
                match split[0] {
                    "$" => match split[1] {
                        "cd" => {
                            let path = split[2];
                            if path == ".." {
                                path_stack.pop();
                                let last_size = size_stack.pop().unwrap_or(0);
                                *size_stack.last_mut().unwrap_or(&mut 0) += last_size;
                                if last_size < MAX_SIZE {
                                    result += last_size;
                                }
                            } else {
                                path_stack.push(path);
                                size_stack.push(0);
                            }
                        }
                        _ => {}
                    },
                    "dir" => {}
                    size @ _ => {
                        let size = size.parse::<i32>().unwrap();
                        *size_stack.last_mut().unwrap_or(&mut 0) += size;
                    }
                };
                (result, path_stack, size_stack)
            },
        )
        .0
}

fn part_2(input: &str) -> i32 {
    const TOTAL_SPACE: i32 = 70000000;
    const NEED_SPACE: i32 = 30000000;
    let mut path_stack = Vec::new();
    let mut direcory_sizes = HashMap::new();
    let mut folders = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        if split[0] == "$" {
            if let ["cd", path] = split[1..] {
                if path == ".." {
                    path_stack.pop();
                } else {
                    path_stack.push(path);
                }
            }
        } else if let ["dir", dir_name] = split[..] {
            folders.push(path_stack.join("/") + "/" + dir_name);
        } else if let [size, _file] = split[..] {
            let size = size.parse::<i32>().unwrap();
            let path = path_stack.join("/");
            *direcory_sizes.entry(path).or_insert(0) += size;
        }
    }
    folders.sort_unstable_by_key(|folder| folder.split('/').count());

    for folder in folders.iter().rev() {
        // Add the size of the folder to the size of the parent folder
        let size = direcory_sizes.get(folder).unwrap_or(&0).clone();
        direcory_sizes
            .entry((&folder[..folder.rfind('/').unwrap()]).to_string())
            .and_modify(|f| *f += size)
            .or_insert(size);
    }

    let free_space = TOTAL_SPACE - *direcory_sizes.get("/").unwrap();
    return *direcory_sizes
        .values()
        .filter(|&size| free_space + size >= NEED_SPACE)
        .min()
        .unwrap();
}
