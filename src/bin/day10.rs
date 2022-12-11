use std::fs;

fn diffs_from_instructions(instructions: &str) -> impl Iterator<Item = i32> + '_ {
    let instructions = instructions.lines();

    let diffs = instructions
        .into_iter()
        .flat_map(
            |line| 
            if line.len() > 4 {
                let add = line.split_at(5).1.parse::<i32>().unwrap();
                vec![0,add]
            } else {
                vec![0]
            });

    diffs
}

fn signal_strengths(instructions: &str) -> impl Iterator<Item = i32> + '_ {
    let diffs = diffs_from_instructions(instructions);

    diffs.scan(1, |x, diff| { *x = *x + diff ; Some(*x) })
}

fn main() {
    let input= fs::read_to_string("/Users/arthurvl/Prive/hobby/code/aoc/2022/input/day10/input")
        .expect("Unable to read day 10 input");

    println!("{:#?}", sum_signal_strengths(&input));
//    println!("{:#?}", calculate_rope_positions(&input));
}

#[cfg(test)]
#[test]
fn example_1() {
    let result: Vec<i32> = signal_strengths("noop
addx 3
addx -5").collect();

    assert_eq!(result,Vec::from([1,1,4,4,-1]));
}

fn sum_signal_strengths(instructions: &str) -> i32 {
    let strengths = signal_strengths(instructions).collect::<Vec<i32>>();

    //dbg!(strengths.clone());

    let interesting_strengths = strengths.into_iter()
        .enumerate()
        .filter(|(i,_)| i < &230 && i % 40 == 18)
        .map(|(i,s)| ((i+2) as i32)*s);

    interesting_strengths.sum()
}

#[test]
fn example_2() {
    let result = sum_signal_strengths("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");

    assert_eq!(result,13140);
}

#[test]
fn example_3() {
}