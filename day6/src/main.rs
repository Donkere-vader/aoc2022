use std::{fs::read_to_string, thread};

const NUM_THREADS: usize = 8;

fn detect_unique_sequence(data: &[char], sequence_length: usize) -> Result<usize, String> {
    let mut buffer: Vec<char> = vec!['a'];
    buffer.append(&mut data.iter().take(sequence_length - 1).map(|c| *c).collect::<Vec<char>>());
    let mut skip_check: usize = 0;
    'outer: for (idx, character) in data.iter().enumerate().skip(sequence_length - 1) {
        buffer.push(*character);
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

fn do_parallel(data: &[char], sequence_length: usize) -> usize {
    let mut threads = Vec::new();

    let section_size = data.len() / NUM_THREADS;

    for i in 0i32..NUM_THREADS as i32 {
        let start = ((i * section_size as i32) - (sequence_length - 1) as i32).max(0) as usize;
        let end = (i * section_size as i32) as usize + section_size;
        let sub_section = data[start..end].iter()
                .map(|c| *c)
                .collect::<Vec<char>>();
        threads.push(thread::spawn(move || {
            detect_unique_sequence(&sub_section, sequence_length)
        }));
    }

    for join_handle in threads.into_iter() {
        let result = join_handle.join().unwrap();

        if let Ok(num) = result {
            return num;
        }
    }

    panic!("Not found");
}

fn part1(data: &[char]) -> usize {
    do_parallel(data, 4)
}

fn part2(data: &[char]) -> usize {
    do_parallel(&data, 14)
}

fn main() {
    let file_contents = read_to_string("bigboy.txt").unwrap();
    let data = file_contents.trim();
    let chars = data.chars().collect::<Vec<char>>();
    let result_1 = part1(&chars);

    println!("Part 1: {}", result_1);
    let result_2 = part2(&chars);
    println!("Part 2: {}", result_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test_1() {
        assert_eq!(part1(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect::<Vec<char>>()), 7);
    }

    #[test]
    fn part_1_test_2() {
        assert_eq!(part1(&"bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect::<Vec<char>>()), 5);
    }

    #[test]
    fn part_1_test_3() {
        assert_eq!(part1(&"nppdvjthqldpwncqszvftbrmjlhg".chars().collect::<Vec<char>>()), 6);
    }

    #[test]
    fn part_1_test_4() {
        assert_eq!(part1(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect::<Vec<char>>()), 10);
    }

    #[test]
    fn part_1_test_5() {
        assert_eq!(part1(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect::<Vec<char>>()), 11);
    }

    #[test]
    fn part_2_test_1() {
        assert_eq!(part2(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect::<Vec<char>>()), 19);
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(part2(&"bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect::<Vec<char>>()), 23);
    }

    #[test]
    fn part_2_test_3() {
        assert_eq!(part2(&"nppdvjthqldpwncqszvftbrmjlhg".chars().collect::<Vec<char>>()), 23);
    }

    #[test]
    fn part_2_test_4() {
        assert_eq!(part2(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect::<Vec<char>>()), 29);
    }

    #[test]
    fn part_2_test_5() {
        assert_eq!(part2(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect::<Vec<char>>()), 26);
    }
}
