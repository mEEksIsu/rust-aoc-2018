use crate::days::Day;
use std::fs;
use std::collections::HashSet;

pub struct Day1 {
    input: String,
}

impl Day1 {
    pub fn new() -> Day1 {
        let input = fs::read_to_string("res/day1.txt").unwrap();

        Day1 { input }
    }
}

impl Day for Day1 {
    fn part1(&self) -> String {
        self.input
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let changes: Vec<i32> =
            self.input
                .lines()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

        let mut current_frequency: i32 = 0;
        let mut unique_frequencies = HashSet::new();

        loop {
            for change in &changes {
                if unique_frequencies.contains(&current_frequency) {
                    return current_frequency.to_string()
                } else {
                    unique_frequencies.insert(current_frequency);
                    current_frequency += change;
                }
            }
        }
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day1::new().part1().equals("592"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day1::new().part2().equals("241"));
    }
}
