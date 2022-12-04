use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Sections {
    start: u32,
    end: u32,
}

type Assignments = (Sections, Sections);

impl From<&str> for Sections {
    fn from(value: &str) -> Self {
        let nums_in_value: Vec<&str> = value.split("-").collect();
        let start = nums_in_value[0].parse().unwrap();
        let end = nums_in_value[1].parse().unwrap();

        Sections {
            start,
            end,
        }
    }
}

fn get_input() -> Vec<Assignments> {
    let file_contents = read_to_string("input.txt").unwrap();
    let mut input = Vec::new();

    for line in file_contents.split("\n") {
        if line.is_empty() { continue }
        let pairs_in_line: Vec<&str> = line.split(",").collect();

        input.push((Sections::from(pairs_in_line[0]), Sections::from(pairs_in_line[1])));
    }

    input
}

fn in_between(value: u32, start: u32, end: u32) -> bool {
    start <= value && end >= value
}

fn part2(input: Vec<Assignments>) -> u32 {
    let mut overlaps = 0;

    for assignment in input.into_iter() {
        if in_between(assignment.0.start, assignment.1.start, assignment.1.end)
            || in_between(assignment.0.end, assignment.1.start, assignment.1.end)
            || in_between(assignment.1.start, assignment.0.start, assignment.0.end)
            || in_between(assignment.1.end, assignment.0.start, assignment.0.end) {
            overlaps += 1;
        }
    }

    overlaps
}

fn part1(input: Vec<Assignments>) -> u32 {
    let mut contains = 0;

    for assignment in input.into_iter() {
        if (assignment.0.start <= assignment.1.start && assignment.0.end >= assignment.1.end)
            || (assignment.0.start >= assignment.1.start && assignment.0.end <= assignment.1.end) {
            contains += 1;
        }
    }

    contains
}

fn main() {
    let input = get_input();

    println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input.clone()));
}
