#![allow(dead_code)]

use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy)]

struct Position {
  x: i32,
  y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Size {
  width: i32,
  height: i32,
}

#[derive(Debug, Clone)]
struct PotentialPart {
  value: String,
  length: i32,
  x0: Option<i32>,
  y0: Option<i32>,
  star_position: Option<Position>,
}

type Part = PotentialPart;

fn main() {
  let path = "3.dat";
  let test_input = fs::read_to_string(path).unwrap();

  let indexed_map = text_to_indexed_map(test_input.as_str());

  let max_x = indexed_map[0].len() as i32;
  let max_y = indexed_map.len() as i32;

  let size = Size { width: max_x, height: max_y };

  let potential_parts = test_input
    .lines()
    .enumerate()
    .flat_map(|(y0, line)| extract_potential_parts_from_line(line, y0 as i32));

  let parts = potential_parts.map(|potential_part| add_star_position(potential_part, &size, &indexed_map));

  let parts_with_star: Vec<Part> = parts.filter_map(|part| part).collect();

  let star_location_to_parts_map = generate_star_location_to_parts_map(parts_with_star, size);

  let sum: i32 = star_location_to_parts_map
    .values()
    .filter_map(|part| {
      if part.len() == 2 {
        return Some(part[0].value.parse::<i32>().unwrap() * part[1].value.parse::<i32>().unwrap());
      }
      return None;
    })
    .sum();

  println!("{}", sum);
}

fn generate_star_location_to_parts_map(parts: Vec<Part>, size: Size) -> HashMap<i32, Vec<Part>> {
  let init_map: HashMap<i32, Vec<Part>> = HashMap::new();

  let filled_map = parts.into_iter().fold(init_map, |mut map, part| {
    let star_position = part.star_position.unwrap();
    let index = star_position.x + star_position.y * size.height;

    match map.get_mut(&index) {
      Some(x) => {
        x.push(part);
      }
      None => {
        map.insert(index, vec![part]);
      }
    }
    return map;
  });

  return filled_map;
}

fn add_star_position(mut potential_part: PotentialPart, size: &Size, indexed_map: &Vec<Vec<char>>) -> Option<PotentialPart> {
  let x0 = potential_part.x0.unwrap() as i32;
  let y0 = potential_part.y0.unwrap() as i32;
  let length = potential_part.length as i32;

  let top_line = (x0 - 1..(x0 + length + 1)).map(|x| (x, y0 - 1));
  let bottom_line = (x0 - 1..(x0 + length + 1)).map(|x| (x, y0 + 1));
  let left = std::iter::once((x0 - 1, y0));
  let right = std::iter::once((x0 + length, y0));

  let mut neighbours = top_line
    .chain(bottom_line)
    .chain(left)
    .chain(right)
    .filter(|(x, y)| x >= &0 && x < &size.width && y >= &0 && y < &size.height)
    .map(|(x, y)| Position { x, y });

  let star_position = neighbours.find(|Position { x, y }| indexed_map[*y as usize][*x as usize] == '*');

  return match star_position {
    None => None,
    Some(x) => {
      potential_part.star_position = Some(x);
      Some(potential_part)
    }
  };
}

fn text_to_indexed_map(text: &str) -> Vec<Vec<char>> {
  return Vec::from_iter(text.lines().map(|x| Vec::from_iter(x.chars())));
}

fn extract_potential_parts_from_line(line: &str, y0: i32) -> Vec<PotentialPart> {
  let init_potential_parts: Vec<PotentialPart> = vec![init_potential_part(None, Some(y0))];

  let potential_parts = line.chars().enumerate().fold(init_potential_parts, |mut result, (x0, char)| {
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

  let filtered_potential_parts: Vec<PotentialPart> = potential_parts.into_iter().filter(|x| x.length > 0).collect();

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
  return PotentialPart { value: "".to_string(), length: 0, x0, y0, star_position: None };
}
