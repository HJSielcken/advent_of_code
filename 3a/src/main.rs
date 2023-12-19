#![allow(dead_code)]

use std::clone::Clone;

#[derive(Debug, Clone)]
struct PotentialPart {
    value: String,
    length: usize,
    x0: Option<usize>,
    y0: Option<usize>,
}

fn main() {
    let test_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    let potential_parts: Vec<PotentialPart> = test_input
        .lines()
        .enumerate()
        .flat_map(|(y0, line)| extract_potential_parts_from_line(line, y0))
        .collect();

    println!("{:?}", potential_parts);
}

fn extract_potential_parts_from_line(line: &str, y0: usize) -> Vec<PotentialPart> {
    let init_potential_parts: Vec<PotentialPart> = vec![init_potential_part(None, Some(y0))];

    let potential_parts =
        line.chars()
            .enumerate()
            .fold(init_potential_parts, |mut result, (x0, char)| {
                match char.to_string().parse::<usize>() {
                    Ok(_) => {
                        let potential_part = result
                            .last_mut()
                            .unwrap_or_else(|| panic!("Something went wrong"));

                        update_potential_part(x0, char, potential_part);
                        return result;
                    }
                    Err(_) => {
                        let new_potential_part = init_potential_part(None, Some(y0));
                        result.push(new_potential_part);
                        return result;
                    }
                };
            });

    let filtered_potential_parts: Vec<PotentialPart> = potential_parts
        .into_iter()
        .filter(|x| x.length > 0)
        .collect();

    return filtered_potential_parts;
}



fn update_potential_part(x0: usize, char: char, potential_part: &mut PotentialPart) -> () {
    if potential_part.x0.is_none() {
        potential_part.x0 = Some(x0)
    }

    potential_part.length += 1;
    potential_part.value.push(char);
}

fn init_potential_part(x0: Option<usize>, y0: Option<usize>) -> PotentialPart {
    return PotentialPart {
        value: "".to_string(),
        length: 0,
        x0,
        y0,
    };
}
