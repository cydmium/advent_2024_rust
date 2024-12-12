advent_of_code::solution!(2);
use std::iter::zip;

pub fn parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut input_data: Vec<Vec<isize>> = vec![];
    for line in input.lines() {
        let items: Vec<isize> = line
            .split_ascii_whitespace()
            .map(|value| value.parse::<isize>().unwrap())
            .collect();
        input_data.push(items);
    }
    input_data
}

pub fn check_report(report: &Vec<isize>) -> bool {
    let diffs: Vec<isize> = zip(report[1..].iter(), report[0..report.len() - 1].iter())
        .map(|(element1, element2)| element1 - element2)
        .collect();
    (diffs.iter().all(|diff| *diff > 0) || diffs.iter().all(|diff| *diff < 0))
        && diffs.iter().all(|diff| diff.abs() <= 3)
}

pub fn part_one(input: &str) -> Option<usize> {
    let data: Vec<Vec<isize>> = parse_input(input);
    let result: usize = data
        .iter()
        .map(|report| check_report(report))
        .filter(|x| *x)
        .count();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = parse_input(input);

    let result: usize = data
        .iter()
        .map(|report| {
            (0..report.len()).any(|i| {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                check_report(&report_copy)
            })
        })
        .filter(|x| *x)
        .count();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
