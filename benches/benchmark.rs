use aoc_2018::days::{
    day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, day7::Day7, Day,
};
use criterion::*;

fn criterion_benchmark(c: &mut Criterion) {
    let day1_1 = Day1::new();
    let day1_2 = Day1::new();
    let day2_1 = Day2::new();
    let day2_2 = Day2::new();
    let day3_1 = Day3::new();
    let day3_2 = Day3::new();
    let day4_1 = Day4::new();
    let day4_2 = Day4::new();
    let day5_1 = Day5::new();
    let day5_2 = Day5::new();
    let day6_1 = Day6::new();
    let day6_2 = Day6::new();
    let day7_1 = Day7::new();
    let day7_2 = Day7::new();

    c.bench_function("day1 part1", move |b| b.iter(|| day1_1.part1()));
    c.bench_function("day1 part2", move |b| b.iter(|| day1_2.part2()));
    c.bench_function("day2 part1", move |b| b.iter(|| day2_1.part1()));
    c.bench_function("day2 part2", move |b| b.iter(|| day2_2.part2()));
    c.bench_function("day3 part1", move |b| b.iter(|| day3_1.part1()));
    c.bench_function("day3 part2", move |b| b.iter(|| day3_2.part2()));
    c.bench_function("day4 part1", move |b| b.iter(|| day4_1.part1()));
    c.bench_function("day4 part2", move |b| b.iter(|| day4_2.part2()));
    c.bench_function("day5 part1", move |b| b.iter(|| day5_1.part1()));
    c.bench_function("day5 part2", move |b| b.iter(|| day5_2.part2()));
    c.bench_function("day6 part1", move |b| b.iter(|| day6_1.part1()));
    c.bench_function("day6 part2", move |b| b.iter(|| day6_2.part2()));
    c.bench_function("day7 part1", move |b| b.iter(|| day7_1.part1()));
    c.bench_function("day7 part2", move |b| b.iter(|| day7_2.part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
