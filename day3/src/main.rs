use std::{fs::read_to_string, collections::HashSet};

#[derive(Debug)]
struct Rucksack {
    compartments: [HashSet<char>; 2]
}

fn prioritize(letter: char) -> u32 {
    let modifier = if letter.is_ascii_uppercase() { 38 } else { 96 };
    let letter_value = letter as u32 - modifier;

    letter_value
}

fn get_input() -> Vec<Rucksack> {
    let file_contents = read_to_string("input.txt").unwrap();
    let mut rucksacks = Vec::new();

    for line in file_contents.split("\n") {
        let (left, right) = line.split_at(line.len() / 2);
        rucksacks.push(Rucksack {
            compartments: [left.to_owned().chars().collect(), right.to_owned().chars().collect()],
        });
    }

    rucksacks
}

fn get_common_letter(rucksack: &Rucksack) -> char {
    let mut possible_common_item: Option<char> = None;

    for item in &rucksack.compartments[0] {
        if rucksack.compartments[1].contains(item) {
            possible_common_item = Some(*item);
            break;
        }
    }
    
    if let Some(common_letter) = possible_common_item {
        return common_letter;
    }
    panic!("Appareanlty couldn't assue there was a smilarity.\nrucksack: {:?}", rucksack);
}

fn part1(rucksacks: &Vec<Rucksack>) -> u32 {
    let mut priorities_sum = 0;

    for rucksack in rucksacks {
        let common_item = get_common_letter(rucksack);
        let prioritized_common_item = prioritize(common_item);
        priorities_sum += prioritized_common_item;
    }
    
    priorities_sum
}

fn part2(rucksacks: &Vec<Rucksack>) -> u32 {
    let mut priorities_sum = 0;

    let mut idx = 0;
    loop {
        let batch: Vec<&Rucksack> = rucksacks.iter().skip(idx * 3).take(3).collect();
        if batch.len() != 3 {
            break;
        }
        idx += 1;

        let rucksack_sets: Vec<HashSet<char>> = batch.into_iter().map(|r| {
            let mut set = HashSet::new();

            set.extend(r.compartments[0].clone());
            set.extend(r.compartments[1].clone());

            println!("{:?} {:?}", r, set);
            set
        }).collect();

        println!("");
        println!("{:?}", rucksack_sets);

        let mut common_letter = None;
        for letter in rucksack_sets[0].iter() {
            if rucksack_sets[1].contains(letter) && rucksack_sets[2].contains(letter) {
                common_letter = Some(letter);
            }
        }

        if let Some(common) = common_letter {
            priorities_sum += prioritize(*common);
        }
    }

    priorities_sum
}

fn main() {
    let rucksacks = get_input();

    let part1_result = part1(&rucksacks);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&rucksacks);
    println!("Part 2: {}", part2_result);
}
