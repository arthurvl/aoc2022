use std::fs;
use std::collections::BTreeSet;

fn find_marker(input: &str) -> usize {
    find_marker_with_size(input, 4)
}

fn find_marker_with_size(input: &str, size: usize) -> usize {
    let mut i = size;

    let chars: Vec<char> = input.chars().collect();

    while i < input.len() {
        let first = chars.get(i-size..i).unwrap();

        let seen = BTreeSet::from_iter(first);

//        println!("first {:#?} input {:#?} i {:#?} char {:#?} seen {:#?} seenlen {:#?}", first, input, i, chars[i], seen, seen.len());
        if seen.len() == size {
            break;
        }

        i = i+1;
    }

    i
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day6/input")
        .expect("Unable to read day 6 input");

    println!("{:#?}", find_marker(&input));
    println!("{:#?}", find_marker_with_size(&input,14));
}

#[cfg(test)]
#[test]
fn example_1() {
    let result = find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

    assert_eq!(result,7);
}

#[test]
fn example_2() {
    let result = find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz");
    
    assert_eq!(result,5)
}