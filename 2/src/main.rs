use std::env;
use std::fs;

#[derive(Copy, Clone)]
enum Items {
  Rock,
  Paper,
  Scissor,
}

fn items_to_value(item: Items) -> u32 {
  return match item {
    Items::Rock => 1,
    Items::Paper => 2,
    Items::Scissor => 3,
  };
}

fn compare_items(opponent: Items, play: Items) -> u32 {
  return match (opponent, play) {
    (Items::Rock, Items::Paper) => 6,
    (Items::Scissor, Items::Rock) => 6,
    (Items::Paper, Items::Scissor) => 6,
    (Items::Paper, Items::Rock) => 0,
    (Items::Rock, Items::Scissor) => 0,
    (Items::Scissor, Items::Paper) => 0,
    _ => 3,
  };
}

fn str_to_items(str: &str) -> Option<Items> {
  return match str {
    "A" => Some(Items::Rock),
    "B" => Some(Items::Paper),
    "C" => Some(Items::Scissor),
    "X" => Some(Items::Rock),
    "Y" => Some(Items::Paper),
    "Z" => Some(Items::Scissor),
    _ => None,
  };
}

fn main() {
  let file_path = "./input";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let lists: Vec<Vec<Items>> = contents
    .trim()
    .split("\n")
    .map(|x| x.split(" ").map(|v| str_to_items(v).unwrap()).collect())
    .collect();
  let res: u32 = lists
    .into_iter()
    .map(|x| compare_items(x[0], x[1]) + items_to_value(x[1]))
    .sum();
  println!("{}", res)
}
