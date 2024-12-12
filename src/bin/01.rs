advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn split_input(input: &str) -> (Vec<isize>, Vec<isize>) {
    let mut left: Vec<isize> = vec![];
    let mut right: Vec<isize> = vec![];

    for line in input.lines() {
        let mut items = line.split_ascii_whitespace();
        left.push(items.next().unwrap().parse::<isize>().unwrap());
        right.push(items.next().unwrap().parse::<isize>().unwrap());
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<isize> {
    let (mut left, mut right) = split_input(input);
    left.sort();
    right.sort();

    let result: isize = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<isize> {
    let (left, right) = split_input(input);

    let mut counts = HashMap::new();
    for num in right.iter() {
        match counts.get(&num) {
            Some(count) => counts.insert(num, count + 1),
            None => counts.insert(num, 1),
        };
    }

    let result: isize = left.iter().map(|l| l * *counts.entry(l).or_default()).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
