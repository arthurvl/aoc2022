use std::fs;
use std::collections::HashMap;

fn main() {
    let naive_plays = HashMap::from([
        ("A X",1+3),
        ("A Y",2+6),
        ("A Z",3+0),
        ("B X",1+0),
        ("B Y",2+3),
        ("B Z",3+6),
        ("C X",1+6),
        ("C Y",2+0),
        ("C Z",3+3)
    ]);

    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day2/input")
        .expect("Unable to read day 2 input");

    let lines = input.lines();

    let playresults = lines.clone().into_iter()
        .map(|line| naive_plays.get(line).unwrap_or(&0));

    println!("{}", playresults.sum::<i32>());

    let new_plays = HashMap::from([
        ("A X",3+0),
        ("A Y",1+3),
        ("A Z",2+6),
        ("B X",1+0),
        ("B Y",2+3),
        ("B Z",3+6),
        ("C X",2+0),
        ("C Y",3+3),
        ("C Z",1+6)
    ]);

    let playresults = lines.into_iter()
        .map(|line| new_plays.get(line).unwrap_or(&0));

    println!("{}", playresults.sum::<i32>());
}