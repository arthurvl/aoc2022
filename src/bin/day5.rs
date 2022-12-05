use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct CrateMovement(u32,usize,usize);

#[derive(PartialEq, Eq, Debug)]
struct CrateMovementParseError;

impl FromStr for CrateMovement {
    type Err = CrateMovementParseError;
    fn from_str(s: &str) -> Result<CrateMovement, CrateMovementParseError> {
        let on_spaces = s.split(' ').collect::<Vec<&str>>();

        if on_spaces.len() != 6 {
            Err(CrateMovementParseError)
        } else {
            let size = on_spaces[1].parse::<u32>();
            let from = on_spaces[3].parse::<usize>();
            let to = on_spaces[5].parse::<usize>();

            if size.is_err() || from.is_err() || to.is_err() {
                Err(CrateMovementParseError)
            } else {
                Ok(CrateMovement(size.unwrap(),from.unwrap(),to.unwrap())) 
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct CrateStacks(usize,Vec<Vec<char>>);

impl CrateStacks {
    fn apply(&self, movement: &CrateMovement) -> CrateStacks {
        let CrateMovement(size,from,to) = movement;

        let from_stack: &Vec<char> = &self.1[from - 1];
        let to_stack: &Vec<char> = &self.1[to - 1];

        let mut new_from_stack = from_stack.clone();
        let mut new_to_stack = to_stack.clone();

        for _i in 0..*size {
            let crate_content = new_from_stack.pop().expect("Empty stack!");
            new_to_stack.push(crate_content);
        }

        let mut new_stacks = self.1.clone();
        new_stacks[from - 1] = new_from_stack;
        new_stacks[to - 1] = new_to_stack;

        CrateStacks(self.0, new_stacks)
    }

    fn apply_multiple(&self, movement: &CrateMovement) -> CrateStacks {
        let CrateMovement(size,from,to) = movement;

        let from_stack: &Vec<char> = &self.1[from - 1];
        let to_stack: &Vec<char> = &self.1[to - 1];

        let mut new_from_stack = from_stack.clone();
        let mut new_to_stack = to_stack.clone();
        let mut holding_stack: Vec<char> = Vec::new();

        for _i in 0..*size {
            let crate_content = new_from_stack.pop().expect("Empty stack!");
            holding_stack.push(crate_content);
        }

        holding_stack.reverse();
        new_to_stack.append(&mut holding_stack);

        let mut new_stacks = self.1.clone();
        new_stacks[from - 1] = new_from_stack;
        new_stacks[to - 1] = new_to_stack;

        CrateStacks(self.0, new_stacks)
    }

    fn tops(&self) -> Vec<char> {
        self.1.iter().map(|s| s.clone().pop().unwrap_or(' ')).collect()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct CrateStacksParseError;

impl FromStr for CrateStacks {
    type Err = CrateStacksParseError;
    fn from_str(s: &str) -> Result<CrateStacks, CrateStacksParseError> {
        let mut upside_down = s.lines().rev().collect::<Vec<&str>>();

        let cols = upside_down[0].split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let col_count = cols.len();

        upside_down.remove(0);

        let mut stacks = Vec::new();

        for _i in 0..col_count {
            stacks.push(Vec::new());
        };

        for line in upside_down {
            if line.len() < (col_count * 4 - 1) {
                return Err(CrateStacksParseError);
            }
            for i in 0..col_count {
                let line: Vec<char> = line.chars().collect();
                let char = line[i * 4 + 1];
                
                if char != ' ' {
                    stacks[i].push(char);
                }
            }
        };

        Ok(CrateStacks(col_count,stacks))
    }
}

fn main() {
    let input= fs::read_to_string("/Users/arthur.vanleeuwen/scratch/aoc2022/input/day5/input")
        .expect("Unable to read day 4 input");

    let stacks = run_input_9000(&input);

    println!("{:#?}", String::from_iter(stacks.tops().iter()));

    let stacks = run_input_9001(&input);

    println!("{:#?}", String::from_iter(stacks.tops().iter()));
}

fn run_input_9000(input: &str) -> CrateStacks {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let stacks = parts[0].parse::<CrateStacks>().expect("Failed to parse stacks");
    let movements: Vec<CrateMovement> = parts[1].lines().map(|line| line.parse::<CrateMovement>().expect("Failed to parse movement")).collect();

    let stacks = movements.iter().fold(stacks,|stacks,movement| stacks.apply(movement));

    stacks
}

fn run_input_9001(input: &str) -> CrateStacks {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let stacks = parts[0].parse::<CrateStacks>().expect("Failed to parse stacks");
    let movements: Vec<CrateMovement> = parts[1].lines().map(|line| line.parse::<CrateMovement>().expect("Failed to parse movement")).collect();

    let stacks = movements.iter().fold(stacks,|stacks,movement| stacks.apply_multiple(movement));

    stacks
}

#[cfg(test)]
#[test]
fn move_parses() {
    let result = "move 1 from 3 to 5".parse::<CrateMovement>();
    assert_eq!(result, Result::Ok(CrateMovement(1,3,5)));
}

#[test]
fn stack_parses() {
    let result = 
"        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3".parse::<CrateStacks>();

    assert_eq!(result, Result::Ok(CrateStacks(3,Vec::from([Vec::from([]),Vec::from(['M','C']),Vec::from(['P','D','N','Z'])]))))
}

#[test]
fn moves_crates_correctly_9000() {
    let input =
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    println!("{:#?}", run_input_9000(input));

    let result = run_input_9000(input).tops();
    assert_eq!(result, Vec::from(['C','M','Z']));
}

#[test]
fn moves_crates_correctly_9001() {
    let input =
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    println!("{:#?}", run_input_9001(input));

    let result = run_input_9001(input).tops();
    assert_eq!(result, Vec::from(['M','C','D']));
}