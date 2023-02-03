#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Rucksack(String);

impl Rucksack {
    fn common_item_priority(&self) -> u32 {
        let contents = &self.0;

        let mid = contents.len() / 2;
        let first_compartment = &contents[..mid];
        let second_compartment = &contents[mid..];
        let common_item = Rucksack::common(first_compartment, second_compartment);

        priority(common_item)
    }

    fn common<'a>(lhs: &'a str, rhs: &'a str) -> char {
        let lhs_set: HashSet<char> = lhs.chars().collect();
        let rhs_set: HashSet<char> = rhs.chars().collect();

        lhs_set
            .intersection(&rhs_set)
            .last()
            .expect("No common item found")
            .clone()
    }
}

fn priority(common_item: char) -> u32 {
    if common_item.is_uppercase() {
        common_item as u32 - 'A' as u32 + 27
    } else {
        common_item as u32 - 'a' as u32 + 1
    }
}

fn part1() {
    let sum: u32 = parse_rucksacks()
        .iter()
        .map(|rucksack| rucksack.common_item_priority())
        .sum();
    println!("Sum: {}", sum);
}

#[derive(Debug)]
struct Elf(Vec<Rucksack>);

impl Elf {
    fn common_item_priority(&self) -> u32 {
        let elf_contents = self
            .0
            .iter()
            .map(|ruckrack| Elf::contents_set(ruckrack))
            .fold(Elf::contents_set(&self.0[0]), |acc, set| {
                acc.intersection(&set).cloned().collect()
            });

        let common_item = elf_contents.iter().last().expect("Common item not found");

        priority(*common_item)
    }

    fn contents_set(rucksack: &Rucksack) -> HashSet<char> {
        rucksack.0.chars().collect()
    }
}

fn part2() {
    let elves = parse_elves();
    assert_eq!(100, elves.len());
    let sum: u32 = elves.iter().map(|elf| elf.common_item_priority()).sum();
    println!("{}", sum);
}

fn main() {
    part2();
}

fn parse_rucksacks() -> Vec<Rucksack> {
    std::io::stdin()
        .lines()
        .map(|line| Rucksack(line.expect("String expected")))
        .collect()
}

fn parse_elves() -> Vec<Elf> {
    std::io::stdin()
        .lines()
        .map(|line| Rucksack(line.expect("String expected")))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| Elf(chunk.to_owned()))
        .collect()
}
