use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone)]
enum DiskItem {
    File {
        name: String,
        size: i32,
    },
    Folder {
        name: String,
        children: HashMap<String, DiskItem>,
    },
}

impl DiskItem {
    pub fn get_size(&self) -> i32 {
        match self {
            DiskItem::File { name, size } => *size,
            DiskItem::Folder { name, children } => children.iter().map(|c| c.1.get_size()).sum(),
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
            DiskItem::Folder { name, children } => children.insert(new_child.get_name(), new_child),
            _ => panic!("Can't add children to DiskItem that is not a folder"),
        };
    }

    pub fn get_child(&mut self, child_name: &str) -> &DiskItem {
        match self {
            DiskItem::Folder { name, children } => children.get_mut(child_name).expect(format!("Error getting child with name: {}", name).as_str()),
            _ => panic!("Can't get child from DiskItem that is not a folder"),
        }
    }

    pub fn get_children(&self) -> Vec<DiskItem> {
        if let DiskItem::Folder { name, children } = self {
            return children.iter().map(|c| c.1.clone()).collect();
        }

        panic!("Can't get children on DiskItem variant that is not a folder");
    }

    pub fn is_folder(&self) -> bool {
        match self {
            DiskItem::Folder { name, children } => true,
            _ => false,
        }
    }

    pub fn print_out(&self, indent: usize) {
        for _ in 0..indent { print!("  ") }
        println!("{:?}", self);
        if let DiskItem::Folder { name, children } = self {
            children.iter().for_each(|c| c.1.print_out(indent + 1));
        }
    }
}

fn parse_input(data: &str) -> DiskItem {
    let mut working_dir = vec![DiskItem::Folder {
        name: "/".to_owned(),
        children: HashMap::new(),
    }];

    for line in data.split("\n") {
        if line.starts_with("$ cd") {
            let target = &line[5..line.len()];
            if target == ".." {
                working_dir.pop();
            } else if target == "/" {
                working_dir = vec![working_dir.remove(0)];
            } else {
                let mut current_dir = working_dir[working_dir.len() - 1].clone();
                working_dir.push(current_dir.get_child(target).clone());
            }
        } else if line.starts_with("$ ls")
        {} else {
            let split_line: Vec<&str> = line.split_whitespace().take(2).collect();
            let size = split_line[0];
            let name = split_line[1];

            let new_child: DiskItem;
            if size == "dir" {
                new_child = DiskItem::Folder { name: name.to_string(), children: HashMap::new() };
            } else {
                new_child = DiskItem::File { name: name.to_string(), size: size.parse().unwrap() }
            }
            let idx = working_dir.len() - 1;
            working_dir[idx].add_child(new_child);
        }
    }

    working_dir.remove(0)
}

fn find_matching_dir(input: &DiskItem, max_size: i32) -> Vec<(DiskItem, i32)> {
    let mut matching_dir = Vec::new();

    for child in input.get_children().iter() {
        if child.is_folder() {
            let size = child.get_size();
            if size < max_size {
                matching_dir.push((child.clone(), size));
            }
        }
    }

    matching_dir
}

fn part1(input: DiskItem) -> i32 {
    let matching_dir: Vec<(DiskItem, i32)> = find_matching_dir(&input, 100_000);

    input.print_out(0);
    println!("{:?}", matching_dir);

    matching_dir.iter().map(|d| d.1).sum()
}

fn main() {
    let file_contents = read_to_string("input.txt").unwrap();
    let input = parse_input(&file_contents);

    println!("Part 1: {}", part1(input.clone()));
}
