use std::fs;
use std::collections::HashSet;

fn update_tail(head: &(i32,i32), tail: &mut (i32,i32)) -> () {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    dbg!(head,tail.clone(),dx,dy);

    if dx.abs() == 2 && dy.abs() == 0 {
        tail.0 = tail.0 + dx.signum();
    }

    if dx.abs() == 0 && dy.abs() == 2 {
        tail.1 = tail.1 + dy.signum();
    }

    if dx.abs() == 2 && dy.abs() == 1 {
        tail.0 = tail.0 + dx.signum();
        tail.1 = head.1;
    }

    if dx.abs() == 1 && dy.abs() == 2 {
        tail.0 = head.0;
        tail.1 = tail.1 + dy.signum();
    }

    dbg!(tail);
}

fn record_tail_positions(movements: &str) -> HashSet<(i32,i32)> {
    let movements = movements.lines();

    let mut head = (0,0);
    let mut tail = (0,0);

    let mut seen_positions = HashSet::new();

    for movement in movements {
        let (dir,steps) = movement.split_at(1);
        let steps = steps.trim().parse::<i32>().unwrap();

        dbg!(dir,steps);

        let dx : i32;
        let dy : i32;

        match dir {
            "R" => { dx = 1; dy = 0; },
            "L" => { dx = -1; dy = 0; },
            "U" => { dx = 0; dy = 1; },
            "D" => { dx = 0; dy = -1; },
            _ => { dx = 0; dy = 0 }
        }

        for _ in 0..steps {
            head.0 = head.0 + dx;
            head.1 = head.1 + dy;

            update_tail(&head,&mut tail);

            seen_positions.insert(tail);
        }

    };

    seen_positions
}

fn calculate_tail_positions(movements: &str) -> usize {
    let positions = record_tail_positions(movements);

    positions.len()
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day9/input")
        .expect("Unable to read day 9 input");

    println!("{:#?}", calculate_tail_positions(&input));
//    println!("{:#?}", find_marker_with_size(&input,14));
}

#[cfg(test)]
#[test]
fn example_1() {
    let result = calculate_tail_positions("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");

    assert_eq!(result,13);
}

#[test]
fn example_2() {
}