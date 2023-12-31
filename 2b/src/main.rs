use std::fs;

#[derive(Debug)]
struct Configuration {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let file = "two.dat";
    let file_content = fs::read_to_string(file);

    let test_input = match file_content {
        Ok(x) => x,
        Err(err) => panic!("{}", err),
    };

    let games = test_input.lines().map(|line| get_rounds(line));

    let minimum_games_configuration = games
        .iter()
        .map(|rounds| minimum_game_configuration(rounds));

    let power_of_cubes = minimum_games_configuration.map(|x|{ x.blue*x.green*x.red});
    let sum_of_power_of_cubes: i32 = power_of_cubes.sum();

    println!("{}", sum_of_power_of_cubes);
}

fn minimum_game_configuration(rounds: &Vec<Configuration>) -> Configuration {
    let init_configuration = Configuration {
        red: 0,
        green: 0,
        blue: 0,
    };

    let configuration = rounds
        .iter()
        .fold(init_configuration, |mut configuration, round| {
            configuration.red = configuration.red.max(round.red);
            configuration.blue = configuration.blue.max(round.blue);
            configuration.green = configuration.green.max(round.green);

            return configuration;
        });

    return configuration;
}

fn get_rounds(line: &str) -> Vec<Configuration> {
    let without_prefix = remove_prefix(line);
    let raw_rounds = without_prefix.split("; ");

    let rounds = raw_rounds.map(|raw_round| get_round(raw_round)).collect();

    return rounds;
}

fn get_round(round: &str) -> Configuration {
    let init_configuration = Configuration {
        red: 0,
        green: 0,
        blue: 0,
    };

    let configuration = round.split(", ").fold(init_configuration, |mut result, x| {
        let count_and_color: Vec<&str> = x.split(" ").collect();
        let (count, color) = match count_and_color.as_slice() {
            [count, color] => (count.parse::<i32>().unwrap_or(0), color.to_string()),
            _ => (0, "".to_string()),
        };

        match color.as_str() {
            "red" => result.red = count,
            "blue" => result.blue = count,
            "green" => result.green = count,
            _ => (),
        }

        return result;
    });

    return configuration;
}

fn remove_prefix(line: &str) -> &str {
    return line
        .split(": ")
        .nth(1)
        .unwrap_or_else(|| panic!("Corrupt input file"));
}
