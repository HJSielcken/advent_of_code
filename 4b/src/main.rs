#[warn(dead_code)]
pub mod read_file;
use read_file::read;
use std::collections::HashMap;

fn main() {
  let path = "4.dat";
  let file_contents = read::read_string_from_file(path);

  let starting_cards_count = get_line_count(&file_contents);

  let cards = file_contents.lines().map(|line| line_to_card(line));

  let matching_numbers_per_card = cards
    .enumerate()
    .map(|(card_index, numbers)| ((card_index + 1) as i32, get_matching_numbers_count(numbers)));

  let total_number_of_scratch_cards: i32 = matching_numbers_per_card
    .fold(HashMap::new(), |mut map, (card_index, score)| {
      let current_card_count_at_root = *map.entry(card_index).or_insert(1);

      let maximum_card_index = starting_cards_count.min(score + card_index);

      return ((card_index + 1)..(maximum_card_index + 1)).fold(map, |mut result, x| {
        let current_card_count = *result.entry(x).or_insert(1);
        let new_card_count = current_card_count + current_card_count_at_root;
        result.insert(x, new_card_count);
        return result;
      });
    })
    .into_values()
    .sum();

  println!("{}", total_number_of_scratch_cards);
}

fn get_matching_numbers_count(numbers: (Vec<i32>, Vec<i32>)) -> i32 {
  let (elf, mine) = numbers;

  let binned_elf_numbers = bin_entries(elf, HashMap::new());
  let binned_elf_and_mine_numbers = bin_entries(mine, binned_elf_numbers);
  return binned_elf_and_mine_numbers.into_values().filter(|x| x == &2).collect::<Vec<i32>>().len() as i32;
}

fn bin_entries(entries: Vec<i32>, initial_map: HashMap<i32, i32>) -> HashMap<i32, i32> {
  return entries.iter().fold(initial_map, |mut result, x| {
    let value = result.get(x).unwrap_or(&0);
    result.insert(*x, value + 1);

    return result;
  });
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

fn get_line_count(file_contents: &str) -> i32 {
  return file_contents.lines().collect::<Vec<&str>>().len() as i32;
}
