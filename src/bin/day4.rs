use std::fs;

use itertools::Itertools;

#[derive(Debug)]
struct ElfAssignment(u32,u32);

fn elf_assignment_for(elfdescr: &str) -> ElfAssignment {
    let (lwb,upb) = elfdescr
        .split('-')
        .map(|bound| bound.parse::<u32>().unwrap())
        .into_iter()
        .collect_tuple::<(u32,u32)>()
        .unwrap();

    ElfAssignment(lwb, upb)
}

fn extract_elf_pair(line: &str) -> (ElfAssignment,ElfAssignment) {
    let (first,second) = line.split(',').collect_tuple().unwrap();
    (elf_assignment_for(first), elf_assignment_for(second))
}

fn contains(left: &ElfAssignment, right: &ElfAssignment) -> bool {
       left.0 <= right.0 && right.1 <= left.1 
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day4/input")
        .expect("Unable to read day 4 input");

    let lines = input.lines();

    let pairs = lines.into_iter()
        .map(extract_elf_pair);

    let duplicating_pairs = pairs
        .filter(|(left,right)| contains(left,right) || contains(right,left));

    println!("{:#?}", duplicating_pairs.count());
}