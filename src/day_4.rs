use std::collections::HashSet;
use crate::utils::file::aggregate_lines;
use crate::utils::output::print_solution;

fn parse_line(line: &str) -> Result<(&str, &str), &'static str>  {
    let card_split = line.split(": ");

    match card_split.last() {
        Some(draws_as_string) => {
            let card_draws: Vec<&str> = draws_as_string.split(" | ").collect();
            match (card_draws.first(), card_draws.last()) {
                (Some(winning_numbers), Some(draw)) => {
                    Ok((winning_numbers, draw))
                }
                _ => {
                    Err("Error getting draws")
                }
            }
        }
        None => {
            Err("Error splitting line")
        }
    }
}

fn get_score((winning_nums_str, draw_str): (&str, &str)) -> usize {
    let winning_nums: HashSet<usize> = winning_nums_str.split_whitespace().map(|x| x.parse::<usize>().ok().unwrap()).collect();

    let winning_drawn_nums: Vec<usize> = draw_str.split_whitespace()
        .map(|x| x.parse::<usize>().ok().unwrap())
        .filter(|x| winning_nums.contains(x))
        .collect();

    if winning_drawn_nums.len() == 0 {
        return 0
    }

    let mut sum = 1;
    for _i in 0..winning_drawn_nums.len() - 1 {
        sum *= 2;
    }
    sum
}

pub fn part_1_solution() {
    let sum = aggregate_lines("inputs/day_4_input.txt", |agg, line| {
        if let Ok(draws) = parse_line(line) {
            return Some(agg + get_score(draws))
        }
        None
    }, 0);
    print_solution(4, 1, sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let parsed_line = parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert!(parsed_line.is_ok());
        assert_eq!(parsed_line.unwrap(), ("41 48 83 86 17", "83 86  6 31 17  9 48 53"));

        let parsed_line_2 = parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert!(parsed_line_2.is_ok());
        assert_eq!(parsed_line_2.unwrap(), (" 1 21 53 59 44", "69 82 63 72 16 21 14  1"));


    }

    #[test]
    fn test_get_score() {
        assert_eq!(get_score(("41 48 83 86 17", "83 86  6 31 17  9 48 53")), 8);
        assert_eq!(get_score((" 1 21 53 59 44", "69 82 63 72 16 21 14  1")), 2);
    }
}