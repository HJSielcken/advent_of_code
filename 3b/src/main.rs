#![allow(dead_code)]

use std::fs;

#[derive(Debug, Clone)]
struct PotentialPart {
    value: String,
    length: i32,
    x0: Option<i32>,
    y0: Option<i32>,
    neighbours: Vec<(i32, i32)>,
    star_position: Option<(i32, i32)>,
}

fn main() {
    //     let test_input = r#"467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598.."#;

    let path = "3.dat";
    let test_input = fs::read_to_string(path).unwrap();

    let indexed_map = text_to_indexed_map(test_input.as_str());

    let max_x = indexed_map[0].len() as i32;
    let max_y = indexed_map.len() as i32;

    let potential_parts = test_input
        .lines()
        .enumerate()
        .flat_map(|(y0, line)| extract_potential_parts_from_line(line, y0 as i32));

    let parts = potential_parts
        .map(|potential_part| determine_neighbours(potential_part, max_x, max_y))
        .filter(|potential_part| determine_if_is_real_part(potential_part, &indexed_map));

    let parts_with_star: Vec<PotentialPart> =
        parts.filter(|part| has_star_as_neighbour(part)).collect();

    let part_couples: Vec<(PotentialPart, PotentialPart)> = find_parts_with_couple(parts_with_star);

    let part_gear_ratios = part_couples.iter().map(|(part_1, part_2)| {
        part_1.value.parse::<i32>().unwrap() * part_2.value.parse::<i32>().unwrap()
    });


    // let sum_of_parts: i32 = parts.map(|part| part.value.parse::<i32>().unwrap()).sum();
    let sum_of_gear_rations: i32 = part_gear_ratios.sum();

    println!("{}", sum_of_gear_rations);
}

fn find_parts_with_couple(parts: Vec<PotentialPart>) -> Vec<(PotentialPart, PotentialPart)> {
    let part_couples: Vec<(PotentialPart, PotentialPart)> = Vec::new();
    return part_couples;
}

fn has_star_as_neighbour(part: &PotentialPart) -> bool {
    return false;
}

fn determine_if_is_real_part(potential_part: &PotentialPart, indexed_map: &Vec<Vec<char>>) -> bool {
    let neighbours = &potential_part.neighbours;
    return neighbours
        .iter()
        .map(|(x, y)| indexed_map[*y as usize][*x as usize])
        .find(|x| x != &'.')
        .is_some();
}

fn determine_neighbours(
    mut potential_part: PotentialPart,
    max_x: i32,
    max_y: i32,
) -> PotentialPart {
    let x0 = potential_part.x0.unwrap() as i32;
    let y0 = potential_part.y0.unwrap() as i32;
    let length = potential_part.length as i32;

    let top_line = (x0 - 1..(x0 + length + 1)).map(|x| (x, y0 - 1));
    let bottom_line = (x0 - 1..(x0 + length + 1)).map(|x| (x, y0 + 1));
    let left = std::iter::once((x0 - 1, y0));
    let right = std::iter::once((x0 + length, y0));

    let neighbors = top_line
        .chain(bottom_line)
        .chain(left)
        .chain(right)
        .filter(|(x, y)| x >= &0 && x < &max_x && y >= &0 && y < &max_y);

    potential_part.neighbours = neighbors.collect();

    return potential_part;
}

fn text_to_indexed_map(text: &str) -> Vec<Vec<char>> {
    return Vec::from_iter(text.lines().map(|x| Vec::from_iter(x.chars())));
}

fn extract_potential_parts_from_line(line: &str, y0: i32) -> Vec<PotentialPart> {
    let init_potential_parts: Vec<PotentialPart> = vec![init_potential_part(None, Some(y0))];

    let potential_parts =
        line.chars()
            .enumerate()
            .fold(init_potential_parts, |mut result, (x0, char)| {
                match char.to_string().parse::<usize>() {
                    Ok(_) => {
                        let potential_part = result.last_mut().unwrap();

                        update_potential_part(x0 as i32, char, potential_part);
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

fn update_potential_part(x0: i32, char: char, potential_part: &mut PotentialPart) -> () {
    if potential_part.x0.is_none() {
        potential_part.x0 = Some(x0)
    }

    potential_part.length += 1;
    potential_part.value.push(char);
}

fn init_potential_part(x0: Option<i32>, y0: Option<i32>) -> PotentialPart {
    return PotentialPart {
        value: "".to_string(),
        length: 0,
        x0,
        y0,
        neighbours: Vec::new(),
        star_position: None
    };
}
