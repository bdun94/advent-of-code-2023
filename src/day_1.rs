use std::collections::HashMap;
use trie_rs::{Trie, TrieBuilder};
use crate::utils::file::aggregate_lines;
use crate::utils::output::print_solution;


/**
  Part 1
**/
fn convert_chars_to_number(chars: (char, char)) -> Option<i32> {
    let (first_digit, last_digit) = chars;
    let formatted_number = format!("{}{}", first_digit, last_digit);
    formatted_number.parse::<i32>().ok()
}

fn get_number_from_line_part_1(line: &str) ->  Option<i32> {
    let first_digit_search =  line.chars().find(|i | i.is_numeric());
    let last_digit_search = line.chars().rfind(|i| i.is_numeric());

    match (first_digit_search, last_digit_search) {
        (Some(first_digit), Some(last_digit)) => {
            convert_chars_to_number((first_digit, last_digit))
        }
        _ => {
            None
        }
    }
}

pub fn part_1_solution() {
    let potential_result = aggregate_lines("inputs/day_1_input.txt", |agg, cur| {
        if let Some(num_from_line) = get_number_from_line_part_1(cur) {
             return Some(agg + num_from_line)
         }
        None
    }, 0);

    print_solution(1, 1, potential_result);
}

/**
  Part 2
**/

fn create_digit_trie() -> Trie<u8> {
    let mut builder = TrieBuilder::new();  // Inferred `TrieBuilder<u8>` automatically
    builder.push("one");
    builder.push("two");
    builder.push("three");
    builder.push("four");
    builder.push("five");
    builder.push("six");
    builder.push("seven");  // Word `push`ed twice is just ignored.
    builder.push("eight");
    builder.push("nine");
    builder.build()
}

fn create_digit_map() -> HashMap<&'static str, char> {
    HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ])
}

fn attempt_slice_translation(slice: &str, trie: &Trie<u8>, digit_map: &HashMap<&str, char>) -> Option<char> {
    for i in 0..slice.len() {
        let check_slice = &slice[0..i + 1];

        // Check if there's an immediate match, if so return it
        match digit_map.get(check_slice) {
            Some(value) => {
                return Some(*value)
            }
            None => {
                // Otherwise check if it's possible to find a digit
                let has_results = !trie.predictive_search(&check_slice).is_empty();
                if !has_results {
                    return None
                }
            }
        }
    }
    None
}

fn get_digits_from_numerals_and_words(line: &str, trie: &Trie<u8>, digit_map: &HashMap<&str, char>) -> Vec<char> {
    let chars = line.chars();
    let mut list = Vec::new();
    for (i, c) in chars.enumerate() {
        if c.is_numeric() {
            list.push(c);
        } else {
            if let Some(digit) = attempt_slice_translation(&line[i..], &trie, &digit_map) {
                list.push(digit);
            }
        }
    }
    list
}

fn get_number_from_digit_list(digits: &Vec<char>) -> Option<i32> {
    if digits.is_empty() {
        return None;
    }

    if let Some(last_digit) = digits.last().copied() {
        return convert_chars_to_number((digits[0], last_digit));
    }
    None
}

pub fn part_2_solution() {
    let trie = create_digit_trie();
    let digit_map = create_digit_map();

    let aggregate_attempt = aggregate_lines("inputs/day_1_input.txt", |agg, line| {
        let digits =  get_digits_from_numerals_and_words(line, &trie, &digit_map);
        if let Some(number) = get_number_from_digit_list(&digits) {
            return Some(agg + number)
        }
        None
    }, 0);

    print_solution(1, 2, aggregate_attempt);
}
