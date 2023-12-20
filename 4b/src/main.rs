#[warn(dead_code)]
pub mod read_file;
use std::collections::HashMap;

use read_file::read;

fn main() {
  let path = "4.dat";
  let file_contents = read::read_string_from_file(path);

  let number_of_starting_cards = get_line_count(&file_contents);
  let init_map = generate_init_map(number_of_starting_cards);

  let score_per_card: Vec<(i32, i32)> = file_contents
    .lines()
    .map(|line| line_to_card(line))
    .enumerate()
    .map(|(i, cards)| ((i + 1) as i32, calculate_match_count(&cards)))
    .collect();

  let number_of_scratch_cards: i32 = score_per_card
    .into_iter()
    .fold(init_map, |map, (card_number, score)| {

      let multiplier = map.get(&card_number).unwrap();
      let max_card_to_add = number_of_starting_cards.min(score + card_number);

      return ((card_number + 1)..(max_card_to_add + 1)).fold(map.to_owned(), |mut result, x| {
        let current_card_count = result.get(&x).unwrap();
        let new_card_count = current_card_count + multiplier * 1;
        result.insert(x, new_card_count);
        return result;
      });
    })
    .into_values()
    .sum();

  println!("{:?}", number_of_scratch_cards);
}

fn calculate_match_count(cards: &(Vec<i32>, Vec<i32>)) -> i32 {
  let init_map: HashMap<i32, i32> = HashMap::new();

  let (elf, me) = cards;
  let map_with_elf_values = elf.iter().fold(init_map, |mut map, x| {
    let value = map.get(x).unwrap_or(&0).clone();
    map.insert(*x, value + 1);
    return map;
  });

  let map_with_all_values = me.iter().fold(map_with_elf_values, |mut map, &x| {
    let value = map.get(&x).unwrap_or(&0).to_owned();
    map.insert(x, value + 1);
    return map;
  });

  let matches_count: i32 = map_with_all_values.into_values().fold(0, |result, x| {
    if x == 2 {
      return result + 1;
    }
    return result;
  });

  return matches_count;
}

fn line_to_card(line: &str) -> (Vec<i32>, Vec<i32>) {
  let line_without_prefix = remove_prefix(line);
  let mut card_sets_iter = line_without_prefix.split("|").map(|set| set.trim()).map(|set| string_to_set(set));

  let elf = card_sets_iter.next().unwrap();
  let me = card_sets_iter.next().unwrap();

  return (elf, me);
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

fn generate_init_map(number_of_cards: i32) -> HashMap<i32, i32> {
  let init_map: HashMap<i32, i32> = HashMap::new();
  return (1..=number_of_cards).fold(init_map, |mut map, x| {
    map.insert(x as i32, 1);
    return map;
  });
}

fn get_line_count(file_contents: &str) -> i32 {
  return file_contents.lines().collect::<Vec<&str>>().len() as i32;
}
