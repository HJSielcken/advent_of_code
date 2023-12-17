use std::fs;

fn main() {
    let filename = "one.dat";
    let input_result = fs::read_to_string(filename);
    let input_content = match input_result {
        Ok(x) => x,
        Err(error) => panic!("Could not read file: {}", error),
    };

    // let input_contents = String::from("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
    let normalized_content = normalize_content(input_content);


    let sum: i32 = normalized_content
        .lines()
        .map(|line| first_integer(line.to_string()) + &last_integer(line.to_string()))
        .map(|x| x.parse::<i32>().unwrap())
        .sum();
    println!("{:?}", sum);
}

fn first_integer(line: String) -> String {
    return line
        .chars()
        .find(|c| c.to_digit(10).is_some())
        .unwrap_or('0')
        .to_string();
}

fn last_integer(line: String) -> String {
    return line
        .chars()
        .rev()
        .find(|c| c.to_digit(10).is_some())
        .unwrap_or('0')
        .to_string();
}

fn normalize_content(content: String) -> String {
    let replace_tabel = [
        ("one".to_string(), "1".to_string()),
        ("two".to_string(), "2".to_string()),
        ("three".to_string(), "3".to_string()),
        ("four".to_string(), "4".to_string()),
        ("five".to_string(), "5".to_string()),
        ("six".to_string(), "6".to_string()),
        ("seven".to_string(), "7".to_string()),
        ("eight".to_string(), "8".to_string()),
        ("nine".to_string(), "9".to_string()),
    ];

    let normalized_content = replace_tabel
        .into_iter()
        .fold(content, |result, (string, int)| {
            result.replace(&string, &(string.to_owned() + &int + &string))
        });

    return normalized_content;
}
