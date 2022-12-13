use std::{fs, cmp::Ordering};
use itertools::Itertools;
use json;

fn part1(input: &str) -> u32 {
    let pairs: Vec<Vec<&str>> = get_pairs(input);

    let indices =
        pairs.into_iter()
            .map(|pair| compare_packets(pair))
            .enumerate()
            .filter(|(_,result)| result.unwrap().is_le() )
            .map(|(i,_)| i + 1);

    indices.sum::<usize>() as u32
}

fn compare_packets(pair: Vec<&str>) -> Option<Ordering> {
    if pair.len() >= 2 {
        let left = pair[0];
        let right = pair[1];

        let left: json::JsonValue = parse_packet(left);
        let right: json::JsonValue = parse_packet(right);

        let compared = compare_jsonvalues(left.clone(),right.clone());

//        dbg!(left,right,compared);

        return compared
    }

    None
}

fn compare_jsonvalues(left: json::JsonValue, right: json::JsonValue) -> Option<Ordering> {
        match left {
            json::JsonValue::Number(left) => compare_number(left,right),
            json::JsonValue::Array(left) => compare_array(left,right),
            _ => None
        }
}

fn compare_array(left: Vec<json::JsonValue>, right: json::JsonValue) -> Option<Ordering> {
    match right {
        json::JsonValue::Number(right) => compare_arrays(left, vec![json::JsonValue::Number(right)]),
        json::JsonValue::Array(right) => compare_arrays(left, right),
        _ => None
    }
}

fn compare_arrays(left: Vec<json::JsonValue>, right: Vec<json::JsonValue>) -> Option<Ordering> {
    if left.is_empty() {
        if right.is_empty() {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    } else if right.is_empty() {
        Some(Ordering::Greater)
    } else {
        let first = compare_jsonvalues(left[0].clone(), right[0].clone());
        if first.is_none() {
            None
        } else {
            if first.unwrap().is_eq() {
                let (_,left) = left.split_first().unwrap();
                let (_,right) = right.split_first().unwrap();
                compare_arrays(Vec::from(left),Vec::from(right))
            } else {
                first
            }
        }
    }
}

fn compare_number(left: json::number::Number, right: json::JsonValue) -> Option<Ordering> {
    match right {
        json::JsonValue::Number(right) => compare_numbers(left,right),
        json::JsonValue::Array(right) => compare_arrays(vec![json::JsonValue::Number(left)],right),
        _ => None
    }
}

fn compare_numbers(left: json::number::Number, right: json::number::Number) -> Option<Ordering> {
    let left = f64::from(left);
    let right = f64::from(right);

    Some(left.total_cmp(&right))
}


fn parse_packet(packet: &str) -> json::JsonValue {
    json::parse(packet).unwrap()
}

fn get_pairs(input: &str) -> Vec<Vec<&str>> {
    input.lines()
        .into_iter()
        .group_by(|&line| line != "")
        .into_iter()
        .filter(|(matches, _)| *matches)
        .map(|(_,group)| group.collect::<Vec<&str>>())
        .collect()
}

fn part2(input: &str) -> u32 {
    let mut packets: Vec<json::JsonValue> = input.lines()
        .into_iter()
        .filter(|line| !line.eq(&""))
        .map(|line| parse_packet(line))
        .collect();

    let two = parse_packet("[[2]]");
    let six = parse_packet("[[6]]");

    packets.push(two.clone());
    packets.push(six.clone());

    packets.sort_by(|left,right| compare_jsonvalues(left.clone(),right.clone()).unwrap());

    let two_index = packets.iter().position(|e| e.eq(&two)).unwrap() + 1;
    let six_index = packets.iter().position(|e| e.eq(&six)).unwrap() + 1;

    (two_index * six_index) as u32
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day13/input")
        .or(fs::read_to_string("/Users/arthurvl/Prive/hobby/code/aoc/2022/input/day13/input"))
        .expect("Unable to read day 13 input");

    println!("{:#?}", part1(&input.clone()));
    println!("{:#?}", part2(&input));
}

#[cfg(test)]
#[test]
fn example_1() {
    let result = part1("[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]");

    assert_eq!(result,13);
}

#[test]
fn parse_empty() {
    let result = parse_packet("[]");

    assert_eq!(result, json::JsonValue::Array(Vec::new()));
}

#[test]
fn parse_empty_with_empty() {
    let result = parse_packet("[[]]");

    assert_eq!(result, json::JsonValue::Array(vec![json::JsonValue::Array(Vec::new())]))
}

#[test]
fn parse_single() {
    let result = parse_packet("[3]");

    assert_eq!(result, json::JsonValue::Array(vec![json::JsonValue::Number(json::number::Number::from(3))]));
}

#[test]
fn test_single_pair_3() {
    let result = part1("[9]
[[8,7,6]]");

    assert_eq!(result, 0);
}

#[test]
fn example_2() {
     let result = part2("[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]");

    assert_eq!(result,140);
}

#[test]
fn example_3() {
    let result = "";
    assert_eq!(result,"")
}
