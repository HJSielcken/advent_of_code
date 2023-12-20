#[warn(dead_code)]
pub mod read_file;
use std::collections::HashMap;

use read_file::read;

#[derive(Debug)]
struct Cards {
  me: Vec<i32>,
  elf: Vec<i32>,
}

fn main() {
  let path = "test.dat";
  let file_contents = read::read_string_from_file(path);

  let games: Vec<i32> = file_contents.lines().map(|line| line_to_card(line)).map(|cards| calculate_score(&cards)).collect();
  let score: i32 = file_contents.lines().map(|line| line_to_card(line)).map(|cards| calculate_score(&cards)).into_iter().sum();

  println!("{:?}", games);
  println!("{:?}", score);
}

fn calculate_score(cards: &Cards) -> i32 {
  let init_map: HashMap<i32, i32> = HashMap::new();

  let map_with_elf_values = cards.elf.iter().fold(init_map, |mut map, x| {
    let value = map.get(x).unwrap_or(&0).clone();
    map.insert(*x, value + 1);
    return map;
  });

  let map_with_all_values = cards.me.iter().fold(map_with_elf_values, |mut map, &x| {
    let value = map.get(&x).unwrap_or(&0).to_owned();
    map.insert(x, value + 1);
    return map;
  });

  let matches_count: u32 = map_with_all_values.into_values().fold(0, |result, x| {
    if x == 2 {
      return result + 1;
    }
    return result;
  });

  return match matches_count {
    0 => 0,
    n => {i32::pow(2, n - 1 as u32)}
  };

}

fn line_to_card(line: &str) -> Cards {
  let line_without_prefix = remove_prefix(line);
  let mut card_sets_iter = line_without_prefix.split("|").map(|set| set.trim()).map(|set| string_to_set(set));

  let elf = card_sets_iter.next().unwrap();
  let me = card_sets_iter.next().unwrap();

  return Cards { elf, me };
}

fn string_to_set(string: &str) -> Vec<i32> {
  return string
    .split(" ")
    .filter_map(|x| match x.parse::<i32>() {
      Err(_) => None,
      Ok(x) => Some(x),
    })
    .collect();
}

fn remove_prefix(line: &str) -> &str {
  return line.split(":").nth(1).unwrap();
}
