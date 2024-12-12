advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let multiply_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result = multiply_re
        .captures_iter(input)
        .map(|capture| {
            let (_, [l1, l2]) = capture.extract();
            let l1 = l1.parse::<usize>().unwrap();
            let l2 = l2.parse::<usize>().unwrap();
            l1 * l2
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(don\'t\(\))|(do\(\))").unwrap();
    let multiply_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut do_multiply: bool = true;
    let mut running_total: usize = 0;
    for (_, [capture]) in re.captures_iter(input).map(|capture| capture.extract()) {
        if capture == "don't()" {
            do_multiply = false;
        } else if capture == "do()" {
            do_multiply = true;
        } else if do_multiply {
            let (_, [l1, l2]) = multiply_re.captures(capture).unwrap().extract();
            let l1 = l1.parse::<usize>().unwrap();
            let l2 = l2.parse::<usize>().unwrap();
            running_total += l1 * l2;
        }
    }
    Some(running_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
