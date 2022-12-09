use std::fs;
use std::collections::HashSet;

fn new_tail(head: &(i32,i32), tail: &(i32,i32)) -> (i32,i32) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    let new_tail: (i32,i32); 

    if dx.abs() == 2 && dy.abs() == 0 {
        new_tail = (tail.0 + dx.signum(), tail.1);
    } else if dx.abs() == 0 && dy.abs() == 2 {
        new_tail = (tail.0, tail.1 + dy.signum());
    } else if dx.abs() == 2 && dy.abs() == 1 {
        new_tail = (tail.0 + dx.signum(),  head.1);
    } else if dx.abs() == 1 && dy.abs() == 2 {
        new_tail = (head.0,  tail.1 + dy.signum());
    } else if dx.abs() == 2 && dy.abs() == 2 {
        new_tail = (tail.0 + dx.signum(), tail.1 + dy.signum());
    } else {
        new_tail = (tail.0,tail.1);
    }

//    dbg!(tail,dx,dy,new_tail);

    new_tail
}

fn record_tail_positions(movements: &str, tailsize: usize) -> HashSet<(i32,i32)> {
    let movements = movements.lines();

    let mut head = (0,0);
    let mut tails = Vec::new();
    
    for _ in 0..tailsize {
        tails.push((0,0));
    }

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

            let new = new_tail(&head, &tails[0]);
            tails[0] = new;

            for i in 1..tailsize {
                let new_tail = new_tail(&tails[i-1], &tails[i]);

                tails[i] = new_tail;
            };

            dbg!(head,tails.clone());

            seen_positions.insert(tails[tailsize-1]);
        };

    };

    seen_positions
}

fn calculate_tail_positions(movements: &str) -> usize {
    let positions = record_tail_positions(movements,1);

    positions.len()
}

fn calculate_rope_positions(movements: &str) -> usize {
    let positions = record_tail_positions(movements,9);

    positions.len()
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day9/input")
        .expect("Unable to read day 9 input");

    println!("{:#?}", calculate_tail_positions(&input));
    println!("{:#?}", calculate_rope_positions(&input));
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
    let result = calculate_rope_positions("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");

    assert_eq!(result,1);
}

#[test]
fn example_3() {
    let result = calculate_rope_positions("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");

    assert_eq!(result,36);
}