use std::cmp::{max, min};
use std::num::ParseIntError;
use std::path::Path;
use crate::utils::file::read_lines;
use crate::utils::output::print_solution;

fn load_file_into_matrix<P>(filename: P) -> Result<Vec<Vec<char>>, &'static str> where P: AsRef<Path> {
    let mut matrix = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            match line {
                Ok(code) => {
                    matrix.push(code.chars().collect())
                }
                Err(_) => {
                    return Err("Error reading the file, please try again");
                }
            }
        }
    }
    Ok(matrix)
}

fn get_rows_to_scan(row: usize, matrix: &Vec<Vec<char>>) -> Vec<usize> {
    if row == 0 {
        return vec![row + 1]
    }

    if row == matrix.len() -1  {
        return vec![row - 1]
    }

    return vec![row - 1, row + 1]
}



fn space_is_numeric(index: usize, row: &Vec<char>) -> bool {
    return match row.get(index) {
        Some(c) => {
            c.is_numeric()
        }
        None => {
            false
        }
    }
}

fn scan_adjacent_areas(row_position: usize, row_index: usize, current_row: &Vec<char>, rows_to_scan: &Vec<usize>, matrix: &Vec<Vec<char>>) -> bool {
    let mut ptr = row_position;

    let front_and_back_rows_to_scan: Vec<usize> = [&rows_to_scan[..], &vec![row_index][..]].concat();

    // check front if row isn't 0
    if row_position != 0 {
        for row_i in front_and_back_rows_to_scan.iter() {
            let char = matrix[*row_i][ptr - 1];
            if char != '.' && !char.is_numeric() {
                return true
            }
        }
    }


    while space_is_numeric(ptr, current_row) {
        for row_i in rows_to_scan {
            let char = matrix[*row_i][ptr];
            if char != '.' {
                return true;
            }
        }
        ptr += 1;
    }

    // check end
    if ptr < current_row.len() - 1 {
        for row_i in front_and_back_rows_to_scan.iter() {
            let char = matrix[*row_i][ptr];
            if char != '.' && !char.is_numeric() {
                return true
            }
        }
    }
    false
}

fn process_line(row_index: usize, rows_to_scan: &Vec<usize>, matrix: &Vec<Vec<char>>) -> Vec<i32> {
    let mut relevant_row_nums: Vec<i32> = Vec::new();
    let mut ptr = 0;
    if let Some(row) = matrix.get(row_index) {
        while ptr < row.len() {
            if scan_adjacent_areas(ptr, row_index, row, rows_to_scan, matrix) {
                if let Ok((num, new_index)) = get_number_from_index(ptr, row) {
                    relevant_row_nums.push(num);
                    ptr = new_index
                } else {
                    ptr += 1
                }
            } else {
                ptr += 1
            }
        }
    }
    relevant_row_nums
}

fn get_number_from_index(index: usize, row: &Vec<char>) -> Result<(i32, usize), ParseIntError> {
    let mut num_as_str = String::new();
    let slice = &row[index..];
    for char in slice.iter() {
        if !char.is_numeric() {
            break;
        }
        num_as_str.push(*char);
    }
    match num_as_str.parse::<i32>() {
        Ok(number) => {
            Ok((number, index + num_as_str.len()))
        }
        Err(msg) => {
            Err(msg)
        }
    }
}



pub fn part_1_solution() {
    match load_file_into_matrix("inputs/day_3_input.txt") {
        Ok(matrix) => {
            let nums: Vec<i32> = matrix.iter().enumerate().flat_map(|(row_index, _row)| {
                process_line(row_index, &get_rows_to_scan(row_index, &matrix), &matrix)
            }).collect();

            let answer: i32 = nums.iter().sum();
            print_solution(3, 1, Ok(answer));
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn get_number_from_middle(index: usize, row: &Vec<char>) -> Option<(i32, usize)> {
    if let Some(row) = row.get(index) {
        if !row.is_numeric() {
            return None
        }
    }

    let front_slice = &row[..index];
    let back_slice = &row[index..];

    let front_index = front_slice.iter().rev().position(|x| !x.is_numeric());
    let back_index = back_slice.iter().position(|x| !x.is_numeric());

    match (front_index, back_index) {
        (Some(fi), Some(be)) => {
            let num_string: String = row[index-fi..index + be].iter().collect();
            match num_string.parse::<i32>() {
                Ok(num) => {
                    Some((num, index + be))
                }
                Err(_msg) => {
                    None
                }
            }
        }
        (None, Some(be)) => {
            let num_string: String = row[..index + be].iter().collect();
            match num_string.parse::<i32>() {
                Ok(num) => {
                    Some((num, index + be))
                }
                Err(_msg) => {
                    None
                }
            }
        }
        (Some(fi), None) => {
            let num_string : String = row[index-fi..].iter().collect();
            match num_string.parse::<i32>() {
                Ok(num) => {
                    Some((num, row.len()))
                }
                Err(_msg) => {
                    None
                }
            }
        }
        (None, None) => {
            None
        }
    }
}

fn check_gear(col_index: usize, row_index: usize, matrix: &Vec<Vec<char>>) -> Option<Vec<i32>> {
    // Don't check if the current char is not a gear
    if matrix[row_index][col_index] != '*' {
        return None
    }

    let top_and_bottom = vec![row_index-1, row_index +1];
    let mut nums: Vec<i32> = Vec::new();

    // scan the top and bottom
    for row in top_and_bottom {
        let mut ptr = max(col_index - 1, 0);

        while ptr <= min(col_index + 1, matrix[0].len() - 1) {
            if space_is_numeric(ptr, &matrix[row]) {
                if let Some((num, new_index)) = get_number_from_middle(ptr, &matrix[row]) {
                    nums.push(num);
                    ptr = new_index;
                }
                else {
                    ptr += 1;
                }
            } else {
                ptr += 1;
            }
        }
    }

    // scan the sides
    if col_index > 0 {
        if space_is_numeric(col_index - 1, &matrix[row_index]) {
            if let Some((num, _new_index)) = get_number_from_middle(col_index - 1, &matrix[row_index]) {
                nums.push(num);
            }
        }
    }

    if col_index < matrix[row_index].len() - 1 {
        if space_is_numeric(col_index + 1, &matrix[row_index]) {
            if let Some((num, _new_index)) = get_number_from_middle(col_index + 1, &matrix[row_index]) {
                nums.push(num);
            }
        }
    }

    if nums.len() == 2 {
        return Some(nums)
    }
    None
}

pub fn part_2_solution() {
    let mut sum = 0;
    match load_file_into_matrix("inputs/day_3_input.txt") {
        Ok(matrix) => {
            for (row_index, row)  in matrix.iter().enumerate().filter(|(row_index, _row)| *row_index != 0 || *row_index != matrix.len() - 1) {
                for (col_index, col) in row.iter().enumerate() {
                    if *col == '*' {
                        if let Some(gears) = check_gear(col_index, row_index, &matrix) {
                            sum += gears.iter().product::<i32>();
                        }
                    }
                }
            }
            print_solution(3, 2, Ok(sum));
        }
        Err(msg) => {
            println!("{}",msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_matrix() -> Vec<Vec<char>> {
        vec![
            "467..114..".chars().collect(),
            "...*......".chars().collect(),
            "..35..633.".chars().collect(),
            "......#...".chars().collect(),
            "617*......".chars().collect(),
            ".....+.58.".chars().collect(),
            "..592.....".chars().collect(),
            "......755.".chars().collect(),
            "...$.*....".chars().collect(),
            ".664.598..".chars().collect(),
        ]
    }

    #[test]
    fn test_get_number() {
        let test_input_1: Vec<char> = "467..114..".chars().collect();
        let test_input_2: Vec<char> = "617*......".chars().collect();
        let test_input_3: Vec<char> = ".......755".chars().collect();

        let test_1 = get_number_from_index(0, &test_input_1);
        assert!(test_1.is_ok());
        assert_eq!(test_1.unwrap(), (467, 3));

        let test_2 = get_number_from_index(5, &test_input_1);
        assert!(test_2.is_ok());
        assert_eq!(test_2.unwrap(), (114, 8));

        let test_3 = get_number_from_index(0, &test_input_2);
        assert!(test_3.is_ok());
        assert_eq!(test_3.unwrap(), (617, 3));


        let test_4 = get_number_from_index(7, &test_input_3);
        assert!(test_4.is_ok());
        assert_eq!(test_4.unwrap(), (755, 10));
    }

    #[test]
    fn test_process_line() {
        let matrix = create_test_matrix();

        let test_cases = vec![
            (0, vec![467]),
            (1, vec![]),
            (2, vec![35, 633]),
            (3, vec![]),
            (4, vec![617]),
            (5, vec![]),
            (6, vec![592]),
            (7, vec![755]),
            (8, vec![]),
            (9, vec![664, 598]),
        ];

        for (row, answers) in test_cases.iter() {
            let nums = process_line(*row, &get_rows_to_scan(*row, &matrix), &matrix );

            assert_eq!(nums, *answers);
        }
    }

    #[test]
    fn test_get_number_from_middle() {
        let test_input_1: Vec<char> = "467..114..".chars().collect();
        let test_input_2: Vec<char> = "617*......".chars().collect();
        let test_input_3: Vec<char> = ".......755".chars().collect();

        assert_eq!(get_number_from_middle(2, &test_input_1).unwrap(), (467, 3));
        assert_eq!(get_number_from_middle(0, &test_input_1).unwrap(), (467, 3));
        assert_eq!(get_number_from_middle(5, &test_input_1).unwrap(), (114, 8));
        assert_eq!(get_number_from_middle(6, &test_input_1).unwrap(), (114, 8));
        assert_eq!(get_number_from_middle(7, &test_input_1).unwrap(), (114, 8));
        assert_eq!(get_number_from_middle(0, &test_input_2).unwrap(), (617, 3));
        assert_eq!(get_number_from_middle(8, &test_input_3).unwrap(), (755, 10));
        assert_eq!(get_number_from_middle(7, &test_input_3).unwrap(), (755, 10));
        assert_eq!(get_number_from_middle(9, &test_input_3).unwrap(), (755, 10));

    }

    #[test]
    fn test_check_gear() {
        let matrix = create_test_matrix();
        let gear_check = check_gear(3, 1, &matrix);
        assert!(gear_check.is_some());
        assert_eq!(gear_check.unwrap(), vec![467, 35]);

        let gear_check_2 = check_gear(5, 8, &matrix);
        assert!(gear_check_2.is_some());
        assert_eq!(gear_check_2.unwrap(), vec![755, 598]);

        let gear_check_3 = check_gear(3, 4, &matrix);
        assert!(gear_check_3.is_none());

    }
}