use std::fs::read_to_string;

fn get_input() -> Vec<Vec<u32>> {
    let file_contents = read_to_string("input.txt").unwrap();
    let mut elves_calories = Vec::new();

    let mut new_elf = Vec::new();
    for line in file_contents.split("\n") {
        if line.len() > 0 {
            new_elf.push(line.parse().unwrap());
        } else {
            elves_calories.push(new_elf.clone());
            new_elf.clear();
        }
    };
    if new_elf.len() > 0 {
        elves_calories.push(new_elf.clone());
    }

    elves_calories
}

fn get_totals(elves_calories: &Vec<Vec<u32>>) -> Vec<u32> {
    elves_calories.clone().iter().map(|elf| elf.iter().sum()).collect()
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let totals = get_totals(input);

    let mut biggest = 0;
    for elf in totals.iter() {
        if elf > &biggest {
            biggest = *elf;
        }
    }

    biggest
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut totals = get_totals(input);
    totals.sort();

    totals.iter().rev().take(3).sum()
}

fn main() {
    let input = get_input();
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}
