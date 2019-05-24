use crate::days::Day;
use std::collections::HashMap;
use std::fs;

pub struct Day2 {
    input: String,
}

impl Day2 {
    pub fn new() -> Day2 {
        let input = fs::read_to_string("res/day2.txt").unwrap();

        Day2 { input }
    }
}

impl Day for Day2 {
    fn part1(&self) -> String {
        (self.input
            .lines()
            .map(|x| is_char_repeated_in_id(&x.to_string(), 2))
            .filter(|x| *x)
            .count() *
        self.input
            .lines()
            .map(|x| is_char_repeated_in_id(&x.to_string(), 3))
            .filter(|x| *x)
            .count()).to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

fn is_char_repeated_in_id(id: &String, repeat_count: u32) -> bool {
    let mut frequency: HashMap<char, u32> = HashMap::new();

    for ch in id.chars() {
        let count = frequency.entry(ch).or_insert(0);
        *count += 1;
    }

    frequency.values()
        .find(|&val| *val == repeat_count)
        .is_some()
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day2::new().part1() == "6150");
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day2::new().part2().ends_with("wlkigsqyfecjqqmnxaktdrhbz"));
    }
}
