use std::collections::HashMap;
use crate::days::{Day, day1::Day1, day2::Day2};

pub struct DayRunner {
    days: HashMap<u32, Box<Day>>
}

impl DayRunner {
    pub fn new() -> DayRunner {
        let mut days = HashMap::<u32, Box<Day>>::with_capacity(25);
        days.insert(1, Box::new(Day1::new()));
        days.insert(2, Box::new(Day2::new()));

        DayRunner { days }
    }

    pub fn run_day(&self, day: u32) {
        match self.days.get(&day) {
            Some(day_instance) => {
                println!();
                println!("========== DAY {} ==========", day);
                println!("===== Part 1 =====");
                println!("{}", day_instance.part1());
                println!("===== Part 2 =====");
                println!("{}", day_instance.part2());
            },
            None => println!("I haven't got to that day yet!"),
        };
    }
}
