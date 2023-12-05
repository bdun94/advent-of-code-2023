use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::utils::file::{aggregate_lines, read_lines};
use crate::utils::output::print_solution;

#[derive(Debug, Hash, PartialEq)]
struct ProcessedCardLine {
    id: usize,
    matching_nums: Vec<usize>
}

fn parse_line(line: &str) -> Result<(&str, &str), &'static str>  {
    let card_split = line.split(": ");

    return match card_split.last() {
        Some(draws_as_string) => {
           split_winning_numbers_and_drawn_numbers(draws_as_string)
        }
        None => {
            Err("Error splitting line")
        }
    }
}


fn split_winning_numbers_and_drawn_numbers(number_section: &str) -> Result<(&str, &str), &'static str> {
    let card_draws: Vec<&str> = number_section.split(" | ").collect();
    match (card_draws.first(), card_draws.last()) {
        (Some(winning_numbers), Some(draw)) => {
            Ok((winning_numbers, draw))
        }
        _ => {
            Err("Error getting draws")
        }
    }
}


fn get_matching_numbers(winning_nums_str: &str, draw_str: &str) -> Vec<usize> {
    let winning_nums: HashSet<usize> = winning_nums_str.split_whitespace().map(|x| x.parse::<usize>().ok().unwrap()).collect();

    draw_str.split_whitespace()
        .map(|x| x.parse::<usize>().ok().unwrap())
        .filter(|x| winning_nums.contains(x))
        .collect()
}

fn get_score((winning_nums_str, draw_str): (&str, &str)) -> usize {
    let winning_drawn_nums = get_matching_numbers(&winning_nums_str, &draw_str);

    if winning_drawn_nums.len() == 0 {
        return 0
    }

    winning_drawn_nums[1..].iter().fold(1_usize, |agg, _cur|  agg * 2)
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


fn parse_line_part_2(line: &str) -> Result<(usize, (&str, &str)), &'static str> {
    let card_split: Vec<&str> = line.split(": ").collect();

    return match (card_split.first(), card_split.last()) {
        (Some(card_portion) , Some(numbers_portion)) => {
            return match (card_portion.split_whitespace().last(), split_winning_numbers_and_drawn_numbers(numbers_portion)) {
                (Some(card_number_str), Ok((winning_numbers, drawn_numbers))) => {
                    return match card_number_str.parse::<usize>() {
                        Ok(card_number) => {
                            Ok((card_number, (winning_numbers, drawn_numbers)))
                        }
                        Err(_err) => {
                            Err("Error parsing card number into an integer")
                        }
                    }
                }
                _ => {
                    Err("Error ")
                }
            };
        }
        _ => {
            Err("Error splitting line")
        }
    }
}

fn convert_line_to_card(line: &str) -> Result<ProcessedCardLine, &'static str> {
    match parse_line_part_2(&line) {
        Ok((card_number, (winning_nums_str, drawn_nums_str))) =>  {
            Ok(
                ProcessedCardLine {
                    id: card_number,
                    matching_nums: get_matching_numbers(winning_nums_str, drawn_nums_str)
                }
            )
        }
        Err(msg) => Err(msg)
    }
}

fn load_cards<P>(filename: P) -> Result<Vec<ProcessedCardLine>, &'static str> where P: AsRef<Path> {
    let mut cards: Vec<ProcessedCardLine> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let current_line = match line {
                Ok(current_line) => {
                    current_line
                }
                Err(_) => {
                    return Err("Error reading the file, please try again");
                }
            };

            match convert_line_to_card(&current_line) {
                Ok(processed_card_line) => {
                    cards.push(processed_card_line);
                }
                Err(_msg) => {
                    return Err("Error converting line to dataclass")
                }
            }
        }
    }
    Ok(cards)
}

fn process_cards_for_part_2(cards: &Vec<ProcessedCardLine>) -> HashMap<usize, usize> {
   let mut card_quantity_map: HashMap<usize, usize> = cards.iter().map(|x| (x.id, 1_usize)).collect();
    let total_cards = card_quantity_map.keys().len();

    for card in cards {
        let total_matching_nums = card.matching_nums.len();
        let current_copies = match card_quantity_map.get(&card.id) {
            Some(num) => *num,
            None => 0
        };
        for _i in 0..current_copies {
            for card_id in card.id+1..=min(card.id+total_matching_nums, total_cards) {
                card_quantity_map.entry(card_id).and_modify(|x| *x += 1);
            }
        }
    }
    card_quantity_map
}

pub fn part_2_solution() {
    match load_cards("inputs/day_4_input.txt") {
        Ok(cards) => {
            let total_cards: usize = process_cards_for_part_2(&cards).values().sum();
            print_solution(4, 2, Ok(total_cards));
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
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

    #[test]
    fn test_convert_line_to_card() {
        let card = convert_line_to_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card.unwrap(), ProcessedCardLine {
            id: 1_usize,
            matching_nums: vec![83_usize, 86_usize, 17_usize, 48_usize]
        });
        let card_3 = convert_line_to_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(card_3.unwrap(), ProcessedCardLine {
            id: 3_usize,
            matching_nums: vec![21usize, 1usize]
        });

        let card_5 = convert_line_to_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(card_5.unwrap(), ProcessedCardLine {
            id: 5_usize,
            matching_nums: vec![]
        });
    }

    #[test]
    fn test_process_cards_for_part_2() {
        let cards = vec![
            convert_line_to_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap(),
            convert_line_to_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").unwrap(),
            convert_line_to_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").unwrap(),
            convert_line_to_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").unwrap(),
            convert_line_to_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").unwrap(),
            convert_line_to_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap()
        ];

        let card_quantity_by_id = process_cards_for_part_2(&cards);
        let expected: HashMap<usize, usize> = HashMap::from([
            (1_usize, 1_usize),
            (2_usize, 2_usize),
            (3_usize, 4_usize),
            (4_usize, 8_usize),
            (5_usize, 14_usize),
            (6_usize, 1_usize),
        ]);

        assert_eq!(card_quantity_by_id, expected);
    }
}