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

fn valid_for_checking(index: usize, row: &Vec<char>) -> bool {
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


    while valid_for_checking(ptr, current_row) {
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
            let nums: Vec<i32> = matrix.iter().enumerate().flat_map(|(row_index, row)| {
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
}