use std::fs::read_to_string;


fn detect_unique_sequence(data: &str, sequence_length: usize) -> Result<usize, String> {
    let mut buffer: Vec<char> = vec!['a'];
    buffer.append(&mut data.chars().take(sequence_length - 1).collect::<Vec<char>>());
    let mut skip_check: usize = 0;
    'outer: for (idx, character) in data.chars().enumerate().skip(sequence_length - 1) {
        buffer.push(character);
        buffer.remove(0);
        if skip_check > 0 {
            skip_check -= 1;
            continue;
        }

        for (letter_idx, letter) in buffer.iter().take(sequence_length - 1).enumerate() {
            let mut total_of_letter = 0;
            buffer.iter().for_each(|l| if l == letter { total_of_letter += 1});
            if total_of_letter > 1 {
                skip_check = letter_idx;
                continue 'outer;
            }
        }

        return Ok(idx + 1);
    }

    Err("Not found".to_string())
}


fn part1(data: &str) -> usize {
    detect_unique_sequence(data, 4).unwrap()
}

fn part2(data: &str) -> usize {
    detect_unique_sequence(data, 14).unwrap()
}

fn main() {
    let file_contents = read_to_string("input.txt").unwrap();
    let data = file_contents.trim();
    let result_1 = part1(data);

    println!("Part 1: {}", result_1);
    let result_2 = part2(data);
    println!("Part 2: {}", result_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test_1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn part_1_test_2() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn part_1_test_3() {
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn part_1_test_4() {
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn part_1_test_5() {
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_2_test_1() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }

    #[test]
    fn part_2_test_3() {
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn part_2_test_4() {
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn part_2_test_5() {
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
