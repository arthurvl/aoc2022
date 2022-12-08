use std::fs;

fn parse_forest(trees: &str) -> Vec<Vec<i8>> {
    trees
        .lines()
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_string().parse::<i8>().unwrap()).collect())
        .collect()
}

fn mark_visible(forest: Vec<Vec<i8>>) -> Vec<Vec<bool>> {
    let height = forest.len();
    let width = forest[0].len();

    let mut forest_visibility = Vec::new();

    for (y, row) in forest.iter().enumerate() {
        let mut row_visibility = Vec::new();

        for (x, tree) in row.iter().enumerate() {
            let visibile_to_left =  x == 0 || (0..x).map(|prec| forest[y][prec]).all(|other| other < *tree);
            let visibile_to_right =  x == width-1 || (x+1..width).map(|prec| forest[y][prec]).all(|other| other < *tree);
            let visibile_to_top =  y == 0 || (0..y).map(|prec| forest[prec][x]).all(|other| other < *tree);
            let visibile_to_bottom =  y == height-1 || (y+1..height).map(|prec| forest[prec][x]).all(|other| other < *tree);

            row_visibility.push(visibile_to_top || visibile_to_bottom || visibile_to_left || visibile_to_right);
        }

        forest_visibility.push(row_visibility);
    };

    forest_visibility
}

fn count_visible_from_trees(forest: Vec<Vec<i8>>) -> Vec<Vec<usize>> {
    let height = forest.len();
    let width = forest[0].len();

    let mut forest_visibility = Vec::new();

    for (y, row) in forest.iter().enumerate() {
        let mut row_visibility = Vec::new();

        for (x, tree) in row.iter().enumerate() {
            let visibile_to_left =  if x == 0 { 0 } else { 
                let span: Vec<i8> = (0..x).rev().map(|prec| forest[y][prec]).collect();
                let res = span.iter().take_while(|&other| other < tree).count();
                if res == span.len() { res } else { res + 1 } };
            let visibile_to_right =  if x == width-1 {0} else { 
                let span: Vec<i8> = (x+1..width).map(|prec| forest[y][prec]).collect();
                let res = span.iter().take_while(|&other| other < tree).count();
                if res == span.len() {res} else {res+1} };
            let visibile_to_top =  if y == 0 {0} else { 
                let span: Vec<i8> = (0..y).rev().map(|prec| forest[prec][x]).collect();
                let res = span.iter().take_while(|&other| other < tree).count();
                if res == span.len() { res} else {res + 1} };
            let visibile_to_bottom =  if y == height-1 {0} else { 
                let span: Vec<i8> = (y+1..height).map(|prec| forest[prec][x]).collect();
                let res = span.iter().take_while(|&other| other < tree).count();
                if res == span.len() { res} else {res + 1} };

            row_visibility.push(visibile_to_top * visibile_to_bottom * visibile_to_left * visibile_to_right);
        }

        forest_visibility.push(row_visibility);
    };

    forest_visibility
}

fn count_visible_trees(trees: &str) -> usize {
    let forest = parse_forest(trees);

    let visible = mark_visible(forest);

    visible.into_iter()
        .flat_map(|row| row.into_iter().filter(|&tree_visible| tree_visible))
        .count()
}

fn max_scenic_score(trees: &str) -> usize {
    let forest = parse_forest(trees);
    let scenic_scores = count_visible_from_trees(forest);

    

    dbg!(scenic_scores).into_iter()
        .flat_map(|row| row.into_iter())
        .max()
        .unwrap()
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day8/input")
        .expect("Unable to read day 8 input");

    println!("{:#?}", count_visible_trees(&input));
    println!("{:#?}", max_scenic_score(&input));
}

#[cfg(test)]
#[test]
fn example_1() {
    let result = count_visible_trees("30373
25512
65332
33549
35390");

    assert_eq!(result, 21);
}

#[test]
fn example_2() {
    let result = max_scenic_score("30373
25512
65332
33549
35390");

    assert_eq!(result, 8);
}