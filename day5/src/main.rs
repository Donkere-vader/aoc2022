use std::{fs::read_to_string};
use regex::Regex;

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            crates: Vec::new(),
        }
    }

    pub fn stack(&mut self, new_crate: char) {
        self.crates.push(new_crate);
    }

    pub fn pop(&mut self) -> char {
        self.crates.pop().unwrap()
    }

    pub fn read(&self) -> &char {
        if self.crates.is_empty() {
            return &' ';
        }
        &self.crates[self.crates.len() - 1]
    }
}

#[derive(Debug, Clone)]
struct Move {
    amount: u32,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let mut stacks = Vec::new();
    let mut moves = Vec::new();

    let move_re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let column_nums_re = Regex::new("\\s*([0-9])\\s*").unwrap();
    let crate_re = Regex::new(" ( )  |\\[([A-Z])\\]").unwrap();

    let lines = input.split('\n').collect::<Vec<&str>>();

    for line in lines.into_iter().rev() {
        if line.is_empty() { continue }
        if line.starts_with("move") {
            let captures = move_re.captures(line).unwrap();
            let new_move = Move {
                amount: captures.get(1).unwrap().as_str().parse().unwrap(),
                from: captures.get(2).unwrap().as_str().parse().unwrap(),
                to: captures.get(3).unwrap().as_str().parse().unwrap(),
            };
            moves.push(new_move);
        } else if column_nums_re.is_match(line) {
            for _caps in column_nums_re.captures_iter(line) {
                stacks.push(Stack::new());
            }
        } else if crate_re.is_match(line) {
            for (idx, caps) in crate_re.captures_iter(line).enumerate() {
                if let Some(new_crate) = caps.get(2) {
                    stacks[idx].stack(new_crate.as_str().chars().take(1).collect::<Vec<char>>()[0]);
                }
            }
        }
    }
    moves = moves.into_iter().rev().collect();

    (stacks, moves)
}

fn construct_answer(stacks: &[Stack]) -> String {
    let mut answer = String::new();
    for stack in stacks.iter() {
        answer.push(*stack.read());
    }

    answer
}

fn part1(mut stacks: Vec<Stack>, moves: Vec<Move>) -> String {
    for crane_move in moves.iter() {
        for _ in 0..crane_move.amount {
            let handling_crate = stacks[crane_move.from - 1].pop();
            stacks[crane_move.to - 1].stack(handling_crate);
        }
    }

    construct_answer(&stacks)
}

fn part2(mut stacks: Vec<Stack>, moves: Vec<Move>) -> String {
    for crane_move in moves.iter() {
        let mut handling_crates = Vec::new();
        for _ in 0..crane_move.amount {
            handling_crates.push(stacks[crane_move.from - 1].pop());
        }
        for handling_crate in handling_crates.into_iter().rev() {
            stacks[crane_move.to - 1].stack(handling_crate);
        }
    }

    construct_answer(&stacks)
}

fn main() {
    println!("Reading input...");
    let file_contents = read_to_string("input.txt").unwrap();

    println!("Parsing input...");
    let input = parse_input(&file_contents);

    println!("Calculating...");

    println!("Part 1: {}", part1(input.0.clone(), input.1.clone()));
    println!("Part 2: {}", part2(input.0.clone(), input.1));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> (Vec<Stack>, Vec<Move>) {
        parse_input(&read_to_string("input.txt").unwrap())
    }

    #[test]
    fn test_part1() {
        let input = get_input();
        let res = part1(input.0, input.1);
        assert_eq!(res, "TGWSMRBPN");
    }

    #[test]
    fn test_part2() {
        let input = get_input();
        let res = part2(input.0, input.1);
        assert_eq!(res, "TZLTLWRNF");
    }
}

