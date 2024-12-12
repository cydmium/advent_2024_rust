use std::collections::HashMap;

use std::iter::zip;

advent_of_code::solution!(4);

pub struct Direction {
    x: i32,
    y: i32,
}

pub const N: Direction = Direction { x: 0, y: -1 };
pub const E: Direction = Direction { x: 1, y: 0 };
pub const S: Direction = Direction { x: 0, y: 1 };
pub const W: Direction = Direction { x: -1, y: 0 };
pub const NE: Direction = Direction { x: 1, y: -1 };
pub const NW: Direction = Direction { x: -1, y: -1 };
pub const SE: Direction = Direction { x: 1, y: 1 };
pub const SW: Direction = Direction { x: -1, y: 1 };

pub fn parse_input(input: &str) -> Vec<&str> {
    let mut input_data: Vec<&str> = vec![];
    for line in input.lines() {
        input_data.push(line);
    }
    input_data
}

pub fn part_one(input: &str) -> Option<usize> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| ((x as i32, y as i32), value))
        })
        .collect::<HashMap<(i32, i32), char>>();

    let mas = ['M', 'A', 'S'];
    let result: usize = positions
        .iter()
        .filter(|(_, value)| **value == 'X')
        .map(|(position, _)| {
            [N, E, S, W, NE, NW, SE, SW]
                .iter()
                .map(|direction| {
                    (1..=3).all(|distance| {
                        let new_x = position.0 + direction.x * distance;
                        let new_y = position.1 + direction.y * distance;
                        positions.get(&(new_x, new_y)) == mas.get((distance - 1) as usize)
                    })
                })
                .filter(|x| *x)
                .count()
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| ((x as i32, y as i32), value))
        })
        .collect::<HashMap<(i32, i32), char>>();

    let result: usize = positions
        .iter()
        .filter(|(_, value)| **value == 'A')
        .map(|(position, _)| {
            [[NW, SE], [SE, NW], [NE, SW], [SW, NE]]
                .iter()
                .map(|ms_positions| {
                    zip(ms_positions.iter(), ['M', 'S']).all(|(direction, char)| {
                        let new_x = position.0 + direction.x;
                        let new_y = position.1 + direction.y;
                        positions.get(&(new_x, new_y)) == Some(&char)
                    })
                })
                .filter(|x| *x)
                .count()
                == 2
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
