advent_of_code::solution!(1);
use std::collections::HashMap;

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .trim_end()
        .split('\n')
        .map(|line| digit_extracter(line))
        .sum();
    Some(sum)
}

pub fn digit_extracter(line: &str) -> u64 {
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut left = 0;
    let mut right = line.len() - 1;
    let mut first = false;
    let mut last = false;
    let line: Vec<char> = line.chars().collect();
    loop {
        if first && last {
            break;
        }
        if !first && line[left].is_ascii_digit() {
            first_digit = line[left].to_digit(10).unwrap();
            first = true;
        }
        if !last && line[right].is_ascii_digit() {
            last_digit = line[right].to_digit(10).unwrap();
            last = true;
        }
        if !first && !last {
            right -= 1;
            left += 1;
        } else if last && !first {
            left += 1;
        } else if first && !last {
            right -= 1;
        }
    }

    (first_digit as u64 * 10) + (last_digit as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let string_digit: HashMap<&str, u64> = HashMap::from([
        ("one", 1_u64),
        ("two", 2_u64),
        ("three", 3u64),
        ("four", 4u64),
        ("five", 5u64),
        ("six", 6u64),
        ("seven", 7u64),
        ("eight", 8u64),
        ("nine", 9u64),
    ]);

    let repat_two_digits = Regex::new(r".*?(\d|one|two|three|four|five|six|seven|eight|nine).*(\d|one|two|three|four|five|six|seven|eight|nine).*").unwrap();
    let repat_single_digit =
        Regex::new(r".*?(\d|one|two|three|four|five|six|seven|eight|nine).*").unwrap();
    let s: u64 = input
        .trim_end()
        .split('\n')
        .map(|line| {
            digit_with_words_extractor(line, &repat_two_digits, &repat_single_digit, &string_digit)
        })
        .sum();
    Some(s)
}
pub fn digit_with_words_extractor(
    line: &str,
    repat_two_digits: &Regex,
    repat_single_digit: &Regex,
    string_digit: &HashMap<&str, u64>,
) -> u64 {
    let caps = repat_two_digits.captures(line);

    let first_digit: u64;
    let second_digit: u64;

    if caps.is_none() {
        let caps = repat_single_digit.captures(line).unwrap();
        first_digit = if &caps[1].len() > &1_usize {
            string_digit[&caps[1]]
        } else {
            u64::from(caps[1].chars().next().unwrap().to_digit(10).unwrap())
        };
        return first_digit * 10 + first_digit;
    } else {
        let caps = repat_two_digits.captures(line).unwrap();
        first_digit = if &caps[1].len() > &1_usize {
            string_digit[&caps[1]]
        } else {
            u64::from(caps[1].chars().next().unwrap().to_digit(10).unwrap())
        };
        second_digit = if &caps[2].len() > &1_usize {
            string_digit[&caps[2]]
        } else {
            u64::from(caps[2].chars().next().unwrap().to_digit(10).unwrap())
        };
        return first_digit * 10 + second_digit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142u64));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281u64));
    }
}
