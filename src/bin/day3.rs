use std::fs;
use std::collections::BTreeSet;

use itertools::Itertools;

fn to_priority(letter: char) -> u32 {
    let prio;
    if (letter >= 'a') && (letter <= 'z') {
        prio = u32::from(letter) - u32::from('a') + 1
    } else if (letter >= 'A') && (letter <= 'Z') {
        prio = u32::from(letter) - u32::from('A') + 27
    } else {
        prio = 0
    };
    prio
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day3/input")
        .expect("Unable to read day 3 input");

    let lines = input.lines();

    let split_lines = lines.clone()
        .map(|line| line.split_at(line.len()/2));

    let rucksacks = split_lines
        .map(|(first,second)| (BTreeSet::from_iter(first.chars()), BTreeSet::from_iter(second.chars()) ));

    let uniques = rucksacks
        .map(|(first, second)| { let unique = first.intersection(&second).last().unwrap(); *unique });

    let priorities = uniques.map(to_priority);

    println!("{:#?}", priorities.sum::<u32>());

    let line_groups = lines.map(|line| line.chars()).chunks(3);

    let elf_rucksacks = line_groups
        .into_iter()
        .map(|group| group.map(BTreeSet::from_iter));

    let badges = elf_rucksacks
        .map(|elves| 
                elves
                    .reduce(|acc,x| acc.intersection(&x).cloned().collect::<BTreeSet<char>>() )
                    .unwrap()
                    .into_iter().take(1).collect::<Vec<char>>()[0]
            )
        .collect::<Vec<char>>();

    println!("{:#?}", badges.into_iter().map(to_priority).sum::<u32>());
}