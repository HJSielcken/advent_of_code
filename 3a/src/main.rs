use std::clone::Clone;

#[derive(Debug, Clone)]
struct PotentialPart {
    value: String,
    length: i32,
    x0: i32,
    y0: i32,
}

fn main() {
    let line = "....4534....234...";

    let potential_parts = extract_potential_parts_from_line(line);

    println!("{:?}", potential_parts);
}

fn extract_potential_parts_from_line(line: &str) -> Vec<PotentialPart> {
    let part = init_potential_part(0);

    let init_potential_parts: Vec<PotentialPart> = vec![];

    let potential_parts =
        line.chars()
            .enumerate()
            .fold(init_potential_parts, |mut result, (i, char)| {
                match char.to_string().parse::<i32>() {
                    Ok(x) => {
                        let mut init = part.clone();

                        let potential_part = result.last_mut().unwrap_or(&mut init);

                        potential_part.length += 1;
                        potential_part.value =
                            potential_part.value.to_owned() + x.to_string().as_str();
                        return result;
                    }
                    Err(_) => {
                        let new_potential_part = init_potential_part(i as i32);

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

fn init_potential_part(x0: i32) -> PotentialPart {
    return PotentialPart {
        value: "".to_string(),
        length: 0,
        x0,
        y0: 0,
    };
}
