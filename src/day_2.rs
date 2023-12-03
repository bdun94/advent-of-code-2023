use std::collections::HashMap;
use crate::utils::file::aggregate_lines;
use crate::utils::output::print_solution;

fn create_verification_map() -> HashMap<&'static str, i32> {
    HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ])
}

fn split_game_line(line: &str) -> Result<(i32, &str), &str> {
    let split_line: Vec<&str> = line.split(": ").collect();
    let potential_game = split_line.first();
    let potential_draws = split_line.last();

    return match (potential_game, potential_draws) {
        (Some(game), Some(draws)) => {
            match game.split(" ").last() {
                Some(game_id) => {
                    return match game_id.parse::<i32>() {
                        Ok(game_id_num) => {
                            Ok((game_id_num, draws))
                        }
                        Err(message) => {
                            println!("{}", message.to_string());
                            Err("Error parsing integer, number doesn't seem correct")
                        }
                    }
                }
                None => {
                    Err("Game identifier was not formatted properly")
                }
            }
        }
        _ => {
            Err("Line was not formatted to the specification of the problem input")
        }
    }
}



fn parse_cube(cube: &str) -> Option<(&str, i32)> {
    let parsed_cube: Vec<&str> = cube.split(" ").collect();
    let num_part = parsed_cube.first();
    let color_part = parsed_cube.last();

    return match (num_part, color_part) {
        (Some(&string_num), Some(color)) => {
            match string_num.parse::<i32>() {
                Ok(num) => {
                    Some((color, num))
                }
                _ => {
                    None
                }
            }
        }
        _ => {
            None
        }
    }
}

pub fn get_all_cubes(draws: &str) -> Vec<&str> {
    draws.split("; ").flat_map(|draw| draw.split(", ")).collect()
}

fn verify_game(draws: &str, verification_map: &HashMap<&str, i32>) -> Result<bool, &'static str> {
    for cube in get_all_cubes(draws) {
        match parse_cube(cube)  {
            Some((cube, num)) => {
                match verification_map.get(cube) {
                    Some(total_num) => {
                        if num > *total_num {
                            return Ok(false);
                        }
                    }
                    None => {
                        return Err("Unknown color found in string")
                    }
                }
            }
            _ => {
                return Err("Error parsing cube from draw")
            }
        }
    }
    Ok(true)
}

pub fn part_1_solution() {
    let verification_map = create_verification_map();

    let potential_result = aggregate_lines("inputs/day_2_input.txt", |agg, line| {
        return match split_game_line(line) {
            Ok((game_id, draws)) => {
                match verify_game(draws, &verification_map) {
                    Ok(is_valid) => {
                        if is_valid {
                            return Some(agg + game_id);
                        }
                        None
                    }
                    _ => {
                        None
                    }
                }
            }
            _ => {
                None
            }
        }
    }, 0);

    print_solution(2, 1, potential_result);
}

fn find_minimum_required_for_game(draws: &str) -> HashMap<&str, i32> {
    let mut min_needed_per_color: HashMap<&str, i32> = HashMap::new();
    for cube in get_all_cubes(draws) {
        if let Some((color, current_num_cubes)) = parse_cube(cube){
            match min_needed_per_color.get(color) {
                Some(min_needed) => {
                    if *min_needed < current_num_cubes {
                        min_needed_per_color.insert(color, current_num_cubes);
                    }
                }
                None => {
                    min_needed_per_color.insert(color, current_num_cubes);
                }
            }

        }
    }
    min_needed_per_color
}

pub fn part_2_solution() {
    let potential_result = aggregate_lines("inputs/day_2_input.txt", |agg, line| {
        return match split_game_line(line) {
            Ok((_game_id, draws)) => {
                let min_required_for_game = find_minimum_required_for_game(draws);
                Some(agg + min_required_for_game.values().product::<i32>())
            }
            _ => {
                None
            }
        }
    }, 0);

    print_solution(2, 2, potential_result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_game_line() {
        let line = split_game_line("game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert!(line.is_ok());
        assert_eq!(line.unwrap(), (1, "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
    }

    #[test]
    fn test_parse_cube() {
        let cube = parse_cube("2 green");
        assert!(cube.is_some());
        assert_eq!(cube.unwrap(), ("green", 2));
    }

    #[test]
    fn test_verify_game() {
        let verification_map = create_verification_map();

        // Verify valid result
        let valid_result = verify_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", &verification_map);
        assert!(valid_result.is_ok());
        assert!(valid_result.unwrap());

        let invalid_result = verify_game("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", &verification_map);
        assert!(invalid_result.is_ok());
        assert!(!invalid_result.unwrap())
    }

    #[test]
    fn test_get_minimum_required_for_game_1() {
        let min_cubes_for_game = find_minimum_required_for_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let red_count = min_cubes_for_game.get("red");
        let green_count = min_cubes_for_game.get("green");
        let blue_count = min_cubes_for_game.get("blue");

        assert_eq!(min_cubes_for_game.len(), 3);

        assert!(red_count.is_some());
        assert_eq!(*red_count.unwrap(), 4);

        assert!(green_count.is_some());
        assert_eq!(*green_count.unwrap(), 2);

        assert!(blue_count.is_some());
        assert_eq!(*blue_count.unwrap(), 6);
    }

    #[test]
    fn test_get_minimum_required_for_game_2() {
        let min_cubes_for_game = find_minimum_required_for_game("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        let red_count = min_cubes_for_game.get("red");
        let green_count = min_cubes_for_game.get("green");
        let blue_count = min_cubes_for_game.get("blue");

        assert_eq!(min_cubes_for_game.len(), 3);

        assert!(red_count.is_some());
        assert_eq!(*red_count.unwrap(), 20);

        assert!(green_count.is_some());
        assert_eq!(*green_count.unwrap(), 13);

        assert!(blue_count.is_some());
        assert_eq!(*blue_count.unwrap(), 6);
    }

}