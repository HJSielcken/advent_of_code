use std::fs;

fn main() {
    let filename = "one.dat";
    let input_result = fs::read_to_string(filename);
    let input_contents = match input_result {
        Ok(x) => x,
        Err(error) => panic!("Could not read file: {}", error)
    };

    let sum: i32 = input_contents.lines()
    .map(|line|{first_integer(line.to_string()) + &last_integer(line.to_string())}).map(|x|{x.parse::<i32>().unwrap()}).sum();
    println!("{:?}", sum);

}

fn first_integer(line: String) -> String {
    return line.chars().find(|c| c.to_digit(10).is_some()).unwrap_or('0').to_string();
}

fn last_integer(line: String) -> String {
    return line.chars().rev().find(|c| c.to_digit(10).is_some()).unwrap_or('0').to_string();
}
