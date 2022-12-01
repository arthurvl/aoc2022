use std::fs;

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day1/input")
        .expect("Unable to read day 1 input");

    let lines: Vec<&str> = input.lines().collect();

    let elves = lines.split(|&line| line == "");

    let elves: Vec<Vec<u32>> = elves.map(|elf| elf.iter().map(|&line| line.trim().parse::<u32>().expect("Invalid number of calories found")).collect() ).collect();

    let mut calorietotals: Vec<u32> = elves.iter().map(|elf| elf.iter().sum::<u32>()).collect();

    println!("{:?}", calorietotals.iter().max());

    calorietotals.sort_by(|a,b| b.cmp(a));

    let top = calorietotals.iter().take(3);

    println!("{:?}", top.sum::<u32>());
}
