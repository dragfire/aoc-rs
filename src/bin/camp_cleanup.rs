#![allow(dead_code)]

use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn full_overlap(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
            || other.start <= self.start && other.end >= self.end
    }

    fn partial_overlap(&self, other: &Range) -> bool {
        self.end.max(other.end) - self.start.min(other.start)
            <= (self.end - self.start) + (other.end - other.start)
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.split('-');
        Ok(Range::new(
            v.next().unwrap().parse().unwrap(),
            v.next().unwrap().parse().unwrap(),
        ))
    }
}

fn part1() {
    let pairs: Vec<Vec<Range>> = std::io::stdin()
        .lines()
        .map(|line| {
            line.expect("Pair expected")
                .split(',')
                .map(|v| v.parse::<Range>().unwrap())
                .collect()
        })
        .collect();
    let count = pairs.iter().filter(|p| p[0].full_overlap(&p[1])).count();
    println!("Count: {}", count);
}

fn part2() {
    let pairs: Vec<Vec<Range>> = std::io::stdin()
        .lines()
        .map(|line| {
            line.expect("Pair expected")
                .split(',')
                .map(|v| v.parse::<Range>().unwrap())
                .collect()
        })
        .collect();
    let count = pairs.iter().filter(|p| p[0].partial_overlap(&p[1])).count();
    println!("Count: {}", count);
}

fn main() {
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_overlap() {
        let mut range1 = Range::new(1, 2);
        let mut range2 = Range::new(2, 2);
        assert!(range1.full_overlap(&range2));

        range1 = Range::new(1, 3);
        range2 = Range::new(2, 3);
        assert!(range1.full_overlap(&range2));

        range1 = Range::new(3, 3);
        range2 = Range::new(3, 3);
        assert!(range1.full_overlap(&range2));
    }
}
