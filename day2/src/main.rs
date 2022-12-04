use std::{fs::read_to_string};

#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RoundOutcome {
    Win,
    Draw,
    Lose,
}

impl RoundOutcome {
    pub fn get_score(&self) -> u32 {
        match self {
            RoundOutcome::Win => 6,
            RoundOutcome::Draw => 3,
            RoundOutcome::Lose => 0,
        }
    }
}

impl From<&str> for RoundOutcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => RoundOutcome::Lose,
            "Y" => RoundOutcome::Draw,
            "Z" => RoundOutcome::Win,
            _ => panic!("Invalid value"),
        }
    }
}

impl Hand {
    pub fn get_score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn get_beats_me(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
    
    pub fn get_can_beat(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    pub fn beats(&self, other_hand: &Hand) -> RoundOutcome {
        if self == other_hand {
            return RoundOutcome::Draw;
        }

        if &(self.get_can_beat()) == other_hand {
            return RoundOutcome::Win;
        }

        RoundOutcome::Lose
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" => Hand::Rock,
            "X" => Hand::Rock,
            "B" => Hand::Paper,
            "Y" => Hand::Paper,
            "C" => Hand::Scissors,
            "Z" => Hand::Scissors,
            _ => panic!("Unknown hand {}", value),
        }
    }
}

type Part1Round = [Hand; 2];
type Part2Round = (Hand, RoundOutcome);

fn get_input_part1() -> Vec<Part1Round> {
    let file_contents = read_to_string("input.txt").unwrap();
    let mut rounds = Vec::new();
    
    for line in file_contents.split('\n') {
        if line.is_empty() { continue }
        let split_line: Vec<&str> = line.split(' ').collect();
        rounds.push([
            Hand::from(split_line[0]),
            Hand::from(split_line[1]),
        ]);
    }

    rounds
}

fn get_input_part2() -> Vec<Part2Round> {
    let file_contents = read_to_string("input.txt").unwrap();
    let mut rounds = Vec::new();
    
    for line in file_contents.split('\n') {
        if line.is_empty() { continue }
        let split_line: Vec<&str> = line.split(' ').collect();
        rounds.push((
            Hand::from(split_line[0]),
            RoundOutcome::from(split_line[1]),
        ));
    }

    rounds
}

fn calculate_round_score(round: &Part1Round) -> u32 {
    let mut round_score = 0;

    // hand score
    round_score += round[1].get_score();

    let round_outcome = round[1].beats(&round[0]);
    round_score += round_outcome.get_score();

    round_score
}

fn part1(rounds: &[Part1Round]) -> u32 {
    let mut total = 0;

    for round in rounds.iter() {
        total += calculate_round_score(round);
    }

    total
}

fn part2(rounds: &[Part2Round]) -> u32 {
    let mut total = 0;

    for round in rounds.iter() {
        let my_hand = match round.1 {
            RoundOutcome::Win => round.0.get_beats_me(),
            RoundOutcome::Draw => round.0.clone(),
            RoundOutcome::Lose => round.0.get_can_beat(),
        };

        total += my_hand.get_score() + round.1.get_score();
    }

    total
}

fn main() {
    let rounds_part1 = get_input_part1();
    
    let part1_result = part1(&rounds_part1);
    println!("Part 1: {}", part1_result);

    let rounds_part2 = get_input_part2();
    let part2_result = part2(&rounds_part2);
    println!("Part 2: {}", part2_result);
}

