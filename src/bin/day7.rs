use std::fs;

fn fs_from(cmdlog: &str) -> Vec<u32> {
    let commands = cmdlog.lines();

    let mut result: Vec<u32> = Vec::new();
//    let mut dir_names: Vec<&str> = Vec::new();
    let mut total_sizes: Vec<u32> = Vec::new();
    total_sizes.push(0);

    for line in commands {
        if line.starts_with("$ cd ..") {
            result.push(total_sizes.pop().unwrap());
        } else if line.starts_with("$ cd ") {
            total_sizes.push(0);
        } else if !line.starts_with("$") && ! line.starts_with("dir") {
            let new_size = line.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
            for i in 0..total_sizes.len() {
                total_sizes[i] += new_size;
            };
        }
    };

    while !total_sizes.is_empty() {
        result.insert(0,total_sizes.pop().unwrap());
    }

    result
}


fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day7/input")
        .expect("Unable to read day 7 input");

    let fs = fs_from(&input);

    println!("{:#?}", fs.clone().into_iter().filter(|&dir| dir <= 100000).sum::<u32>());

    let cur_size = fs[0];
    let remaining = 70000000 - cur_size;

    dbg!(fs.clone(), cur_size,remaining);
    let to_delete = 30000000 - remaining;

    let result = fs.into_iter().skip(1).filter(|&dir| dir >= to_delete).min().unwrap();
    println!("{:#?}", result);
}

#[cfg(test)]
#[test]
fn example_1() {
    let result: u32 = fs_from("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k").into_iter().filter(|&dir| dir <= 100000).sum();

    assert_eq!(result, 95437)
}

#[test]
fn example_2() {
    let fs = fs_from("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");

    let cur_size = fs[0];
    let remaining = 70000000 - cur_size;

    dbg!(fs.clone(), cur_size,remaining);
    let to_delete = 30000000 - remaining;

    let result = fs.into_iter().skip(1).filter(|&dir| dir >= to_delete).min().unwrap();
    
    assert_eq!(result,24933642);
}