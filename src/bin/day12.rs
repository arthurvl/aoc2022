use std::{fs, collections::{VecDeque, HashSet}};

fn part1(input: &str) -> u32 {
    let (elevations, start, end) = elevation_field_for(input);

    let mut coords: VecDeque<(usize,usize)> = VecDeque::new();

    // initialize search at start
    coords.push_back(start);

    run_paths(elevations, coords, end)
}

fn run_paths(elevations: Vec<Vec<i32>>,  mut coords: VecDeque<(usize, usize)>, end: (usize, usize)) -> u32 {
    let mut path_lengths: Vec<Vec<u32>> = Vec::new();
    // analyze elevations field, find start and end
    for y in 0..elevations.len() {
        path_lengths.push(Vec::new());
        for _ in 0..elevations[y].len() {
            path_lengths[y].push(u32::MAX);
        }
    };

    for coord in coords.clone() {
        path_lengths[coord.1][coord.0] = 0;
    }

    while !coords.is_empty() {
        let coord = coords.pop_front().unwrap();

    //        println!("Checking {:?}", coord);

        //let deltas = (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (dx,dy).clone()));
        let deltas = [(1,0),(0,1),(-1,0),(0,-1)].iter(); // possible steps

        let neighbors: HashSet<(usize,usize)> = 
            deltas
            .map(|dc| (coord.0 as isize + dc.0, coord.1 as isize + dc.1)) // coordinates after steps
            .filter(|n| n.0 >= 0 && n.1 >= 0 && n.1 < elevations.len() as isize && n.0 < elevations[0].len() as isize) // within the field
            .map(|nc| (nc.0 as usize, nc.1 as usize))
            .collect();

        for neighbor in neighbors {
            if elevations[neighbor.1][neighbor.0] <= elevations[coord.1][coord.0] +1 { // if we can get there

                let length_determined = path_lengths[neighbor.1][neighbor.0];     // see how fast we got there earlier
                let length_taken = path_lengths[coord.1][coord.0] + 1;            // see how fast we get there now
                let new_length = std::cmp::min(length_determined, length_taken); // take the fastest

                if new_length < path_lengths[neighbor.1][neighbor.0] {
                    path_lengths[neighbor.1][neighbor.0] = new_length;
                    if !coords.contains(&neighbor) { coords.push_back(neighbor); };
                }
            }

        }

        // println!("{}\n", Itertools::intersperse(
        //     path_lengths
        //         .clone()
        //         .into_iter()
        //         .enumerate()
        //         .map(|(yc, y)| y.into_iter().enumerate().map(|(xc,x)| 
        //                 if (xc,yc) == coord { "X" 
        //                 } else if coords.contains(&(xc,yc)) { "x"
        //                 } else if x != u32::MAX { "o" } else { "." } ).collect::<String>())
        //     ,"\n".to_string())
        //     .collect::<String>());
    };
    // println!("{}", Itertools::intersperse(
    //         path_lengths
    //             .clone()
    //             .into_iter()
    //             .map(|y| y.into_iter().map(|x| if x != u32::MAX { format!(" {:>3}",x) } else { " ...".to_string() } ).collect::<String>())
    //         ,"\n\n".to_string())
    //         .collect::<String>());
    path_lengths[end.1][end.0]
}

fn elevation_field_for(input: &str) -> (Vec<Vec<i32>>,(usize,usize),(usize,usize))  {
    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    let mut end_x: usize = 0;
    let mut end_y: usize = 0;
    let elevations: Vec<Vec<i32>> = input.lines()
        .into_iter()
        .enumerate()
        .map(|(y,line)| 
            line.chars()
                .enumerate()
                .map(|(x, c)| match c { 
                    'E' => { end_y = y; end_x = x; 26},
                    'S' => { start_x = x; start_y = y; 0},
                    'a'..='z' => c as i32 - 'a' as i32 + 1,
                    _ => -1 } )
                .collect())
        .collect();
    (elevations,(start_x,start_y),(end_x,end_y))
}

fn part2(input: &str) -> u32 {
    let (elevations, _, end) = elevation_field_for(input);

    let startings: Vec<(usize,usize)> = elevations.iter()
        .enumerate()
        .flat_map(|(y,line)| 
            line.iter()
                .enumerate()
                .map(move |(x,&elevation)| if elevation == 1 { Some((x,y)) } else { None })
                .filter(|opt| opt.is_some())
                .map(|opt| opt.unwrap())
        )
        .collect();

    let coords: VecDeque<(usize,usize)> = VecDeque::from(startings);

    run_paths(elevations, coords, end)
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day12/input")
        .or(fs::read_to_string("/Users/arthurvl/Prive/hobby/code/aoc/2022/input/day12/input"))
        .expect("Unable to read day 12 input");

    println!("{:#?}", part1(&input.clone()));
    println!("{}", part2(&input));
}

#[cfg(test)]
#[test]
fn example_1() {
    let result = part1("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi");

    assert_eq!(result,31);
}

#[test]
fn example_2() {
    let result = part2("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi");

    assert_eq!(result,29);
}

#[test]
fn example_3() {
    let result = "";
    assert_eq!(result,"")
}
