use std::{fs::read_to_string};

const DISK_SPACE: u32 = 70_000_000;
const REQUIRED_SPACE: u32 = 30_000_000;

#[derive(Debug, Clone)]
enum DiskItem {
    File {
        name: String,
        size: u32,
    },
    Folder {
        name: String,
        children: Vec<DiskItem>,
    }
}

impl DiskItem {
    pub fn get_size(&self) -> u32 {
        match self {
            DiskItem::File { name, size } => *size,
            DiskItem::Folder { name, children } => {
                children.iter().map(|c| c.get_size()).sum()
            },
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            DiskItem::File { name, size } => name.clone(),
            DiskItem::Folder { name, children } => name.clone(),
        }
    }

    pub fn add_child(&mut self, new_child: DiskItem) {
        match self {
            DiskItem::Folder { name, children } => {
                children.push(new_child);
            },
            DiskItem::File { name, size } => panic!("Cannot add child to file"),
        }
    }

    pub fn get_child(&mut self, child_name: &str) -> &mut DiskItem {
        match self {
            DiskItem::Folder { name, children } => {
                for child in children.iter_mut() {
                    if &child.get_name() == child_name {
                        return child;
                    }
                }
                panic!("No child found with name '{}'", child_name);
            },
            DiskItem::File { name, size } => panic!("Cannot get child from file"),
        }
    }
}

fn parse_input(input: &str) -> DiskItem {
    let mut root = DiskItem::Folder { name: "/".to_string(), children: Vec::new() };

    let mut path: Vec<&str> = Vec::new();
    let mut working_directory = &mut root;

    let mut command;
    for line in input.split('\n').filter(|l| !l.is_empty()) {
        if line.starts_with('$') {
            command = &line[2..line.len()];
            if command.starts_with("cd") {
                let target_directory = command.split_at(2).1.trim();
                if target_directory == "/" {
                    working_directory = &mut root;
                    path = Vec::new();
                } else if target_directory == ".." {
                    path.pop();
                    working_directory = &mut root;
                    for dir_name in path.iter() {
                        working_directory = working_directory.get_child(dir_name);
                    }
                } else {
                    let child = working_directory.get_child(target_directory);
                    working_directory = child;
                    path.push(target_directory);
                }
            }
            continue;
        }
        // command is ls
        let (size, name) = line.split_once(" ").unwrap();
        if size == "dir" {
            working_directory.add_child(DiskItem::Folder {
                name: name.to_string(),
                children: Vec::new(),
            })
        } else {
            working_directory.add_child(DiskItem::File {
                name: name.to_string(),
                size: size.parse::<u32>().unwrap(),
            })
        }
    }

    root
}

fn get_sum_of_matching_folders(disk_item: &DiskItem, max_size: u32) -> Vec<DiskItem> {
    match disk_item {
        DiskItem::Folder { name, children } => {
            let mut all_matching_folders: Vec<DiskItem> = Vec::new();

            for child in children.iter() {
                let mut matching_folders = get_sum_of_matching_folders(child, max_size);
                all_matching_folders.append(&mut matching_folders);
            }

            if disk_item.get_size() < max_size {
                all_matching_folders.push(disk_item.clone());
            }

            all_matching_folders
        },
        DiskItem::File { name, size } => Vec::new(),
    }
}

fn part1(root: DiskItem) -> u32 {
    let all_matching_folders = get_sum_of_matching_folders(&root, 100_000);

    all_matching_folders.iter().map(|f| f.get_size()).sum()
}

fn part2(root: DiskItem) -> u32 {
    let free_space = DISK_SPACE - root.get_size();
    let space_to_free = REQUIRED_SPACE - free_space;
    let mut all_matching_folders = get_sum_of_matching_folders(&root, u32::MAX);
    all_matching_folders = all_matching_folders.into_iter().filter(|f| f.get_size() > space_to_free).collect();

    all_matching_folders.sort_by_key(|f| f.get_size());

    println!("{:?}", all_matching_folders);
    all_matching_folders[0].get_size()
}

fn main() {
    let file_contents = read_to_string("input.txt").unwrap();
    let root = parse_input(&file_contents);

    println!("Part 1: {}", part1(root.clone()));
    println!("Part 2: {}", part2(root));
}
