#![allow(dead_code)]

use std::str::FromStr;

// ---------------- Part 1 --------------------

#[derive(Debug)]
struct Instruction {
    count: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(' ').collect::<Vec<_>>();

        Ok(Instruction {
            count: values[1].parse::<usize>().unwrap(),
            from_stack: values[3].parse::<usize>().unwrap(),
            to_stack: values[5].parse::<usize>().unwrap(),
        })
    }
}

fn part1() {
    let mut lines = std::io::stdin()
        .lines()
        .map(|line| line.expect("String expected"))
        .into_iter();

    let mut grid = parse_grid(lines.by_ref());
    let mut stacks = convert_to_stacks(&mut grid);

    // Skip empty line
    lines.next();

    let instructions = lines
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    for instruction in instructions {
        let containers_from = &mut stacks[instruction.from_stack - 1];
        let mut tail =
            containers_from.split_off(containers_from.len().saturating_sub(instruction.count));
        tail.reverse();
        stacks[instruction.to_stack - 1].append(&mut tail);
    }

    let mut result = String::new();
    for stack in &mut stacks {
        result.push_str(&stack.pop().unwrap().trim_matches(|c| c == '[' || c == ']'));
    }

    println!("Result: {}", result);
}

fn convert_to_stacks(grid: &mut Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut stacks = vec![vec![]; 9];

    for row in grid.iter().rev() {
        for (i, container) in row.iter().enumerate() {
            if container != "   " {
                stacks[i].push(container.to_owned());
            }
        }
    }

    stacks
}

fn parse_grid<I>(iter: I) -> Vec<Vec<String>>
where
    I: IntoIterator<Item = String>,
{
    let mut grid = vec![];
    for line in iter {
        let line = line.trim();
        if line.contains(char::is_numeric) {
            break;
        }
        let row: Vec<String> = parse_string(line);
        grid.push(row);
    }
    grid
}

fn parse_string(s: &str) -> Vec<String> {
    let mut result = vec![];
    let mut i = 0;

    while i < s.len() {
        result.push(s[i..i + 3].to_owned());
        i += 4;
    }

    result
}

// ---------------- Part 2 --------------------

fn part2() {
    let mut lines = std::io::stdin()
        .lines()
        .map(|line| line.expect("String expected"))
        .into_iter();

    let mut grid = parse_grid(lines.by_ref());
    let mut stacks = convert_to_stacks(&mut grid);

    // Skip empty line
    lines.next();

    let instructions = lines
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    for instruction in instructions {
        let containers_from = &mut stacks[instruction.from_stack - 1];
        let mut tail =
            containers_from.split_off(containers_from.len().saturating_sub(instruction.count));
        stacks[instruction.to_stack - 1].append(&mut tail);
    }

    let mut result = String::new();
    for stack in &mut stacks {
        result.push_str(&stack.pop().unwrap().trim_matches(|c| c == '[' || c == ']'));
    }

    println!("Result: {}", result);
}

fn main() {
    part2();
}
