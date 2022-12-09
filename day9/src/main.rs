use std::{fs::read_to_string, collections::HashSet};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn get_move_diff(&self) -> (i32, i32) {
        match self {
            Move::Up => (0, 1),
            Move::Right => (1, 0),
            Move::Down => (0, -1),
            Move::Left => (-1, 0),
        }
    }
}

struct Rope {
    pub head: (i32, i32),
    pub tail: (i32, i32),
}

impl Rope {
    fn apply_move(&mut self, r#move: &Move) {
        let head_diff = r#move.get_move_diff();
        self.head = (
            self.head.0 + head_diff.0,
            self.head.1 + head_diff.1,
        );

        if (self.head.0 - self.tail.0).abs() > 1 || (self.head.1 - self.tail.1).abs() > 1 {
            let tail_diff_relative_head = match r#move {
                Move::Up => (0, -1),
                Move::Right => (-1, 0),
                Move::Down => (0, 1),
                Move::Left => (1, 0),
            };

            self.tail = (
                self.head.0 + tail_diff_relative_head.0,
                self.head.1 + tail_diff_relative_head.1,
            );
        }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("Invalid value"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        let (direction_str, times_str) = line.split_at(1);
        let r#move: Move = direction_str.into();
        let times: u32 = times_str.trim().parse().unwrap();
        for _ in 0..times {
            moves.push(r#move.clone());
        }
    }

    moves
}

fn part1(moves: Vec<Move>) -> usize {
    let mut rope = Rope {
        head: (0, 0),
        tail: (0, 0),
    };
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

    for r#move in moves.iter() {
        rope.apply_move(r#move);
        tail_positions.insert(rope.tail);
    }

    tail_positions.len()
}

fn part2(moves: Vec<Move>) -> usize {
    let mut knots: [(i32, i32); 10] = [(0, 0); 10];
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

    for r#move in moves.iter() {
        let move_diff = r#move.get_move_diff();
        knots[0] = (
            knots[0].0 + move_diff.0,
            knots[0].1 + move_diff.1,
        );

        let mut previous_knot = knots[0];
        let mut previous_jump: Option<(i32, i32)> = None;
        for knot in knots.iter_mut().skip(1) {
            let diff_x = previous_knot.0 - knot.0;
            let diff_y = previous_knot.1 - knot.1;

            if diff_x.abs() > 1 && diff_y == 0 {
                knot.0 += diff_x.clamp(-1, 1);
                previous_jump = None;
            } else if diff_y.abs() > 1 && diff_x == 0 {
                knot.1 += diff_y.clamp(-1, 1);
                previous_jump = None;
            } else if diff_x.abs() > 1 || diff_y.abs() > 1 {
                let jump = match previous_jump {
                    Some(j) => j,
                    None => {
                        let new_jump = (diff_x.clamp(-1, 1), diff_y.clamp(-1, 1));
                        previous_jump = Some(new_jump);

                        new_jump
                    },
                };

                knot.0 += jump.0;
                knot.1 += jump.1;
            } else {
                break;
            }

            previous_knot = *knot;
        }

        tail_positions.insert(knots[knots.len() - 1]);
    }

    tail_positions.len()
}

fn main() {
    let file_contents = read_to_string("bigboy.txt").unwrap();
    let moves = parse_input(&file_contents);

    let mut threads: Vec<JoinHandle<usize>> = Vec::new();
    let moves_1 = moves.clone();
    threads.push(thread::spawn(move || {
        part1(moves_1)
    }));
    threads.push(thread::spawn(move || {
        part2(moves)
    }));

    for (idx, thread) in threads.into_iter().enumerate() {
        println!("Part {}: {}", idx + 1, thread.join().unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test_1() {
        let input_string = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let moves = parse_input(input_string);
        let result = part1(moves);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_2_test_1() {
        let input_string = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let moves = parse_input(input_string);
        let result = part2(moves);
        assert_eq!(result, 1);
    }

    #[test]
    fn part_2_test_2() {
        let input_string = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        let moves = parse_input(input_string);
        let result = part2(moves);
        assert_eq!(result, 36);
    }
}

