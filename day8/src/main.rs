use std::fs::read_to_string;
use colored::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    None,
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug)]
struct Tree {
    pub height: u32,
    pub visible: bool,
    pub checked: bool,
    pub seen_from: Direction,
}

fn parse_input(data_string: &str) -> Vec<Vec<Tree>> {
    let mut matrix = Vec::new();

    for line in data_string.split("\n") {
        matrix.push(line.chars().map(|c| 
                Tree {
                    height: c.to_digit(10).unwrap(),
                    visible: false,
                    checked: false,
                    seen_from: Direction::None,
                }
            ).collect()
        );
    }

    matrix
}

fn print_forest(input: &Vec<Vec<Tree>>) {
    for row in input.iter() {
        for tree in row.iter() {
            let mut text = tree.height.to_string();
            if !tree.visible {
                print!("{}", text.on_red());
            } else {
                print!("{}", text.green());
            }
        }
        println!("");
    }
}

fn part1(mut input: Vec<Vec<Tree>>) -> u32 {
    let mut total_visible = 0;
    let mut work_queue_trees = Vec::new(); // tree coordinates (y, x)
    for i in 0..input.len() {
        let number_of_columns = input[i].len();
        input[i][0].visible = true;
        input[i][number_of_columns - 1].visible = true;
        input[i][0].seen_from = Direction::Horizontal;
        input[i][number_of_columns - 1].seen_from = Direction::Horizontal;
        work_queue_trees.push((i, 0, Direction::Horizontal));
        work_queue_trees.push((i, number_of_columns - 1, Direction::Horizontal));
        total_visible += 2;
    }

    for i in 1..input[0].len() - 1 {
        let number_of_rows = input.len();
        input[0][i].visible = true;
        input[0][i].seen_from = Direction::Vertical;
        input[number_of_rows - 1][i].visible = true;
        input[number_of_rows - 1][i].seen_from = Direction::Vertical;
        work_queue_trees.push((0, i, Direction::Vertical));
        work_queue_trees.push((number_of_rows - 1, i, Direction::Vertical));
        total_visible += 2;
    }

    print_forest(&input);

    while !work_queue_trees.is_empty() {
        let tree_coordinates = work_queue_trees.pop().unwrap();
        println!("{:?}", tree_coordinates);
        let tree_height = input[tree_coordinates.0][tree_coordinates.1].height;
        let tree_seen_from = input[tree_coordinates.0][tree_coordinates.1].seen_from;

        let i_tree_coordinates = (tree_coordinates.0 as i32, tree_coordinates.1 as i32);
        let neighbour_cords: Vec<(usize, usize, Direction)> = vec![
            (i_tree_coordinates.0 + 1, i_tree_coordinates.1, Direction::Vertical),
            (i_tree_coordinates.0 - 1, i_tree_coordinates.1, Direction::Vertical),
            (i_tree_coordinates.0, i_tree_coordinates.1 + 1, Direction::Horizontal),
            (i_tree_coordinates.0, i_tree_coordinates.1 - 1, Direction::Horizontal),
        ].iter()
                .filter(|c| c.0 >= 0 && c.0 < input.len() as i32 && c.1 >= 0 && c.1 < input[0].len() as i32)
                .map(|c| (c.0 as usize, c.1 as usize, c.2))
                .filter(|c| !input[c.0][c.1].visible)
                .filter(|c| c.2 == tree_seen_from)
                .collect();

        for neighbour_cord in neighbour_cords.into_iter() {
            let mut neighbour = &mut input[neighbour_cord.0][neighbour_cord.1];
            work_queue_trees.push(neighbour_cord.clone());
            if neighbour.height > tree_height {
                neighbour.visible = true;
                total_visible += 1;
                if neighbour_cord.2 != neighbour.seen_from && neighbour.seen_from == Direction::None {
                    neighbour.seen_from = tree_seen_from;
                } else if neighbour_cord.2 != neighbour.seen_from {
                    neighbour.seen_from = Direction::Both;
                }
            }
        }
    }

    print_forest(&input);

    total_visible
}

fn main() {
    let file_contents = read_to_string("input.txt").unwrap();
    let input = parse_input(&file_contents);

    println!("Part 1: {}", part1(input));
}

mod test {
    use super::*;

    #[test]
    fn part_1_test_1() {
        let file_string = "30373\n25512\n65332\n33549\n35390";
        let input = parse_input(file_string);
        let result = part1(input);
        assert_eq!(result, 21)
    }
}
