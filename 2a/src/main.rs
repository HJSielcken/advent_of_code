use std::{collections::HashMap, fs};

struct Configuration {
    red: i32,
    green: i32,
    blue: i32,
}

const MAXIMUM_CONFIGURATION: Configuration = Configuration {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let file = "two.dat";
    let file_content = fs::read_to_string(file);

    let test_input = match file_content {
        Ok(x) => x,
        Err(err) => panic!("{}", err),
    };

    let games: Vec<i32> = test_input
        .lines()
        .map(|line| check_game(line))
        .enumerate()
        .filter_map(|(i, result)| match result {
            true => Some((i as i32) + 1),
            false => None,
        })
        .collect();

    let sum: i32 = games.iter().sum();

    println!("{:?}", games);
    println!("{:?}", sum);
}

fn check_game(line: &str) -> bool {
    let without_prefix = remove_prefix(line);
    let rounds = without_prefix.split("; ");

    let mut possible_rounds = rounds.map(|round| check_if_round_is_possible(round));

    let is_possible = possible_rounds.find(|x| x == &false).unwrap_or(true);

    return is_possible;
}

fn check_if_round_is_possible(round: &str) -> bool {
    let empty_map: HashMap<String, i32> = HashMap::new();
    let configurations = round.split(", ");

    let filled_map = configurations.fold(empty_map, |mut map, configuration| {
        let splitted_configuration: Vec<&str> = configuration.split(" ").collect();

        let (count, color) = match splitted_configuration.as_slice() {
            [count, color] => (count.parse::<i32>().unwrap_or(0), color.to_string()),
            _ => (0, "".to_string()),
        };

        map.insert(color, count);
        return map;
    });

    let red_is_possible =
        filled_map.get(&String::from("red")).unwrap_or(&0) <= &MAXIMUM_CONFIGURATION.red;
    let blue_is_possible =
        filled_map.get(&String::from("blue")).unwrap_or(&0) <= &MAXIMUM_CONFIGURATION.blue;
    let green_is_possible =
        filled_map.get(&String::from("green")).unwrap_or(&0) <= &MAXIMUM_CONFIGURATION.green;

    return red_is_possible && green_is_possible && blue_is_possible;
}

fn remove_prefix(line: &str) -> &str {
    return line
        .split(": ")
        .nth(1)
        .unwrap_or_else(|| panic!("Corrupt input file"));
}
