advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut sum: u64 = 0;
    let mut row = 0;
    while row < lines.len() {
        let mut col = 0;
        while col < lines[0].len() {
            if lines[row][col].is_digit(10) {
                let is_valid_number = valid_num(&lines, row, col);
                if (is_valid_number) {
                    let mut left = col;
                    let mut right = col;
                    while ((left > 0) && lines[row][left].is_digit(10)) {
                        left = left - 1;
                    }
                    while ((right < lines[row].len()) && lines[row][right].is_digit(10)) {
                        right += 1
                    }
                    if (left == 0 && lines[row][left].is_digit(10)) {
                        left = 0;
                    } else {
                        left = left + 1;
                    }
                    // if (right == lines[row].len()) {
                    //     right = right - 1;
                    // }
                    // println!(
                    //     "{:?}",
                    //     lines[row][left..right]
                    //         .iter()
                    //         .cloned()
                    //         .collect::<String>()
                    //         .as_str(),
                    // );
                    sum += str::parse::<u64>(
                        lines[row][left..right]
                            .iter()
                            .cloned()
                            .collect::<String>()
                            .as_str(),
                    )
                    .unwrap();
                    col += right - col;
                }
            }
            col += 1;
        }
        row = row + 1;
    }
    // println!("{}", sum);
    Some(sum)
}

pub fn valid_num(lines: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut valid = false;
    //left check
    if col > 0 && lines[row][col - 1].is_ascii_punctuation() && lines[row][col - 1] != '.' {
        valid = true;
    }
    //right
    if col < lines[0].len() - 1
        && lines[row][col + 1].is_ascii_punctuation()
        && lines[row][col + 1] != '.'
    {
        valid = true;
    }
    //up
    if row > 0 && lines[row - 1][col].is_ascii_punctuation() && lines[row - 1][col] != '.' {
        valid = true;
    }
    //down check
    // println!("{}", row);
    if (row < lines.len() - 1)
        && lines[row + 1][col].is_ascii_punctuation()
        && lines[row + 1][col] != '.'
    {
        valid = true;
    }

    //upper-left check
    if (row > 0) & (col > 0)
        && lines[row - 1][col - 1].is_ascii_punctuation()
        && lines[row - 1][col - 1] != '.'
    {
        valid = true;
    }
    //upper-right check
    if (row > 0) & (col < lines[0].len() - 1)
        && lines[row - 1][col + 1].is_ascii_punctuation()
        && lines[row - 1][col + 1] != '.'
    {
        valid = true;
    }
    //lowerleft check
    if (col > 0) & (row < lines.len() - 1)
        && lines[row + 1][col - 1].is_ascii_punctuation()
        && lines[row + 1][col - 1] != '.'
    {
        valid = true;
    }
    //lowerright check
    if (row < lines.len() - 1) & (col < lines[0].len() - 1)
        && lines[row + 1][col + 1].is_ascii_punctuation()
        && lines[row + 1][col + 1] != '.'
    {
        valid = true;
    }
    valid
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    Some(gear_ratio(lines))
}

pub fn parse_num_from_any_position_mat(lines: &Vec<Vec<char>>, row: usize, col: usize) -> u64 {
    let mut left = col;
    let mut right = col;
    while ((left > 0) && lines[row][left].is_digit(10)) {
        left = left - 1;
    }
    while ((right < lines[row].len()) && lines[row][right].is_digit(10)) {
        right += 1
    }
    if (left == 0 && lines[row][left].is_digit(10)) {
        left = 0;
    } else {
        left = left + 1;
    }
    str::parse::<u64>(
        lines[row][left..right]
            .iter()
            .cloned()
            .collect::<String>()
            .as_str(),
    )
    .unwrap()
}

pub fn dfs_on_mat(lines: &Vec<Vec<char>>, row: usize, col: usize) -> (bool, u64, u64) {
    let mut part_count = 0;
    let mut num1: u64 = 0;
    let mut num2: u64 = 0;
    let mut down_taken: bool = false;
    let mut up_taken: bool = false;

    //left check
    if col > 0 && lines[row][col - 1].is_digit(10) {
        part_count += 1;
        if (part_count == 1) {
            num1 = parse_num_from_any_position_mat(lines, row, col - 1);
        } else if part_count == 2 {
            num2 = parse_num_from_any_position_mat(lines, row, col - 1);
        }
    }
    //right
    if col < lines[0].len() - 1 && lines[row][col + 1].is_digit(10) {
        part_count += 1;
        if (part_count == 1) {
            num1 = parse_num_from_any_position_mat(lines, row, col + 1);
        } else if part_count == 2 {
            num2 = parse_num_from_any_position_mat(lines, row, col + 1);
        }
    }

    //up
    if row > 0 && lines[row - 1][col].is_digit(10) {
        part_count += 1;
        up_taken = true;
        if (part_count == 1) {
            num1 = parse_num_from_any_position_mat(lines, row - 1, col);
        } else if part_count == 2 {
            num2 = parse_num_from_any_position_mat(lines, row - 1, col);
        }
    }

    //down check

    // println!("{part_count}, {num1}, {num2}");
    // println!("{}", row);
    if (row < lines.len() - 1) && lines[row + 1][col].is_digit(10) {
        part_count += 1;
        down_taken = true;
        if (part_count == 1) {
            num1 = parse_num_from_any_position_mat(lines, row + 1, col);
        } else if part_count == 2 {
            num2 = parse_num_from_any_position_mat(lines, row + 1, col);
        }
    }

    // println!("{part_count}, {num1}, {num2}");
    //upper-left check
    if !up_taken && (row > 0) & (col > 0) && lines[row - 1][col - 1].is_digit(10) {
        part_count += 1;
        if (part_count == 1) {
            num1 = parse_num_from_any_position_mat(lines, row - 1, col - 1);
        } else if part_count == 2 {
            num2 = parse_num_from_any_position_mat(lines, row - 1, col - 1);
        }
    }

    // println!("{part_count}, {num1}, {num2}");
    //upper-right check
    if !up_taken && (row > 0) & (col < lines[0].len() - 1) && lines[row - 1][col + 1].is_digit(10) {
        part_count += 1;
        if (part_count == 1) {
            num1 = parse_num_from_any_position_mat(lines, row - 1, col + 1);
        } else if part_count == 2 {
            num2 = parse_num_from_any_position_mat(lines, row - 1, col + 1);
        }
    }
    //lowerleft check
    if !down_taken && (col > 0) & (row < lines.len() - 1) && lines[row + 1][col - 1].is_digit(10) {
        part_count += 1;
        if (part_count == 1) {
            num1 = parse_num_from_any_position_mat(lines, row + 1, col - 1);
        } else if part_count == 2 {
            num2 = parse_num_from_any_position_mat(lines, row + 1, col - 1);
        }
    }
    //lowerright check
    if !down_taken
        && (row < lines.len() - 1) & (col < lines[0].len() - 1)
        && lines[row + 1][col + 1].is_digit(10)
    {
        part_count += 1;
        if (part_count == 1) {
            num1 = parse_num_from_any_position_mat(lines, row + 1, col + 1);
        } else if part_count == 2 {
            num2 = parse_num_from_any_position_mat(lines, row + 1, col + 1);
        }
    }

    // println!("{part_count}, {num1}, {num2}");
    if (part_count != 2) {
        (false, num1, num2)
    } else {
        (true, num1, num2)
    }
}

pub fn gear_ratio(mat: Vec<Vec<char>>) -> u64 {
    let cols: usize = mat[0].len();
    let rows: usize = mat.len();
    let mut gear_rat: u64 = 0;
    for i in 0..rows {
        for j in 0..cols {
            if (mat[i][j] == '*') {
                let (gr, g1, g2) = dfs_on_mat(&mat, i, j);
                if gr {
                    gear_rat += g1 * g2;
                }
            }
        }
    }
    gear_rat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
