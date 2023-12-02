use std::collections::HashMap;
use trie_rs::{Trie, TrieBuilder};
use crate::utils::file_utils::aggregate_lines;


fn get_number_from_line_part_1(line: &str) ->  Option<i32> {
    let first_digit_search =  line.chars().find(|i | i.is_numeric());
    let last_digit_search = line.chars().rfind(|i| i.is_numeric());

    match (first_digit_search, last_digit_search) {
        (Some(first_digit), Some(last_digit)) => {
            let formatted_number = format!("{}{}", first_digit, last_digit);
            formatted_number.parse::<i32>().ok()
        }
        _ => {
            None
        }
    }
}

pub fn part_1_solution() -> Result<i32, &'static str> {
    aggregate_lines("inputs/day_1_input.txt", |prev, cur| {
        if let Some(num_from_line) = get_number_from_line_part_1(cur) {
             return Some(prev + num_from_line)
         }
        None
    }, 0)
}

pub fn create_digit_trie() -> Trie<u8> {
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

pub fn create_digit_map() -> HashMap<&'static str, i32> {
    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ].iter().cloned().collect()
}

pub fn attempt_slice_translation(slice: &str, trie: &Trie<u8>, digit_map: &HashMap<&str, i32>) -> Option<i32> {
    let chars = slice.chars();

    for (i, _c) in chars.enumerate() {
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

pub fn get_numbers_from_line(line: &str, trie: &Trie<u8>, digit_map: &HashMap<&str, i32>) -> Vec<i32> {
    let chars = line.chars();
    let mut list = Vec::new();
    for (i, c) in chars.enumerate() {
        if c.is_numeric() {
            if let Some(digit) = c.to_digit(10) {
                list.push(digit as i32);
            }
        } else {
            if let Some(digit) = attempt_slice_translation(&line[i..], &trie, &digit_map) {
                list.push(digit);
            }
        }
    }
    list
}

pub fn part_2_solution(line: &str) {
    let trie = create_digit_trie();
    let digit_map = create_digit_map();
    let digits = get_numbers_from_line(line, &trie, &digit_map);
    println!("{:?}", digits);

}
