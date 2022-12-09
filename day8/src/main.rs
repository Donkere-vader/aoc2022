use std::{fs::read_to_string, collections::HashSet};
use colored::*;

#[derive(Debug, Clone)]
struct Tree {
    pub height: u32,
}

#[derive(Debug)]
struct Ray {
    start: (usize, usize),
    direction: (i32, i32),
}

fn parse_input(data_string: &str) -> Vec<Vec<Tree>> {
    let mut matrix = Vec::new();

    for line in data_string.split("\n") {
        matrix.push(line.chars().map(|c| 
                Tree {
                    height: c.to_digit(10).unwrap(),
                }
            ).collect()
        );
    }

    matrix
}

fn print_forest(input: &Vec<Vec<Tree>>, visible: &HashSet<(usize, usize)>) {
    for (y, row) in input.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let text = tree.height.to_string();
            if visible.contains(&(x, y)) {
                print!("{}", text.green());
            } else {
                print!("{}", text.on_red());
            }
        }
        println!("");
    }
}

fn shoot_ray(ray: Ray, input: &Vec<Vec<Tree>>, max_height: u32, allow_smaller: bool) -> HashSet<(usize, usize)> {
    let mut found_trees: HashSet<(usize, usize)> = HashSet::new();
    let mut ray_height = 0;

    let mut ray_cord = ray.start.clone();

    while ray_cord.0 < input[0].len() && ray_cord.1 < input[1].len() && max_height > ray_height {
        let looking_tree = &input[ray_cord.1][ray_cord.0];
        if ((looking_tree.height > ray_height && !allow_smaller)
                    || (allow_smaller))
                || ray_cord == ray.start {
            found_trees.insert(ray_cord.clone());
            ray_height = looking_tree.height;
        }
        ray_cord = (
            (ray_cord.0 as i32 + ray.direction.0) as usize,
            (ray_cord.1 as i32 + ray.direction.1) as usize,
        );
    }

    found_trees
}

fn part1(input: Vec<Vec<Tree>>) -> usize {
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    let mut rays: Vec<Ray> = Vec::new();
    for i in 0..input.len() {
        rays.push(Ray {
            start: (0, i),
            direction: (1, 0),
        });
        rays.push(Ray {
            start: (input.len() - 1, i),
            direction: (-1, 0),
        });
    }
    for i in 0..input[0].len() {
        rays.push(Ray {
            start: (i, 0),
            direction: (0, 1),
        });
        rays.push(Ray {
            start: (i, input[0].len() - 1),
            direction: (0, -1),
        });
    }

    for ray in rays.into_iter() {
        let trees_found = shoot_ray(ray, &input, 10, false);
        trees_found.into_iter().for_each(|t| {visible_trees.insert(t);});
    }

    visible_trees.len()
}

fn part2(input: Vec<Vec<Tree>>) -> usize {
    let mut scenic_scores = vec![vec![0; input[0].len()]; input.len()];
    let mut highest = 0;

    let forest_height = input.len();
    let forest_width = input[0].len();

    for (y, row) in input.iter().enumerate() {
        'tree: for (x, tree) in row.iter().enumerate() {
            let mut scenic_score = 1;
            for direction in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let neighbour_cord_i32 = (
                    x as i32 + direction.0,
                    y as i32 + direction.1,
                );
                if !(neighbour_cord_i32.0 >= 0
                        && neighbour_cord_i32.0 < forest_width as i32
                        && neighbour_cord_i32.1 >= 0
                        && neighbour_cord_i32.1 < forest_height as i32) {
                    continue 'tree;
                }
                let neighbour_cord = (
                    neighbour_cord_i32.0 as usize,
                    neighbour_cord_i32.1 as usize,
                );
                let trees_found = shoot_ray(Ray {
                    start: neighbour_cord,
                    direction,
                }, &input, tree.height, true);
                scenic_score *= trees_found.len();
            }
            scenic_scores[y][x] = scenic_score;
        }
    }


    for row in scenic_scores.into_iter() {
        for value in row.into_iter() {
            if value > highest {
                highest = value;
            }
        }
    }

    highest
}

fn main() {
    let file_contents = read_to_string("input.txt").unwrap();
    let input = parse_input(&file_contents);

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
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

    #[test]
    fn part_2_test_1() {
        let file_string = "30373\n25512\n65332\n33549\n35390";
        let input = parse_input(file_string);
        let result = part2(input);
        assert_eq!(result, 8)
    }
}
