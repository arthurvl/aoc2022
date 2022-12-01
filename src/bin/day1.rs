use std::fs;
use itertools::Itertools;

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day1/input")
        .expect("Unable to read day 1 input");

    let lines = input.lines();

    let elf_descriptors = lines
        .group_by(|&line| line != "");

    let elves = elf_descriptors
        .into_iter()
        .filter(|(drop,_)| *drop)
        .map(|(_,elf)|  elf.map(|line| line.trim().parse::<u32>().expect("Invalid calorie count")));

    let calorie_totals = elves
        .map(|elf| elf.sum::<u32>())
        .sorted_by(|a,b| b.cmp(a))
        .take(3);

    println!("{}", calorie_totals.clone().max().unwrap_or(0));

    println!("{}", calorie_totals.sum::<u32>());
}
