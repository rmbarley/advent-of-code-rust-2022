#![feature(iter_array_chunks)]
use std::collections::HashMap;

fn calculate_priority() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>()
}

pub fn process_part1(input: &str) -> usize {
    let scores = calculate_priority();
    input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let compartment1 = &line[0..sack_length];
            let compartment2 = &line[sack_length..(sack_length * 2)];

            let common_char = compartment1
                .chars()
                .find(|c| compartment2.contains(*c))
                .unwrap();
            scores.get(&common_char).unwrap()
        })
        .sum::<usize>()
}

pub fn process_part2(input: &str) -> usize {
    let scores = calculate_priority();
    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_char = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();
            scores.get(&common_char).unwrap()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 70);
    }
}
