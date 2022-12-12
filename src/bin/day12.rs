use std::fs;

fn part1(input: &str) -> &str {
    input
}

fn part2(input: &str) -> &str {
    input
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day12/input")
        .expect("Unable to read day 12 input");

    println!("{:#?}", part1(&input.clone()));
    println!("{:#?}", part2(&input));
}

#[cfg(test)]
#[test]
fn example_1() {
    let result = "";
    assert_eq!(result,"");
}

#[test]
fn example_2() {
    let result = 1;
    assert_eq!(result,13140);
}

#[test]
fn example_3() {
    let result = "";
    assert_eq!(result,"")
}
