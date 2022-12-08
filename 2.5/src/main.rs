use std::env;
use std::fs;

#[derive(Copy, Clone)]
enum Items {
  Rock,
  Paper,
  Scissor,
}
#[derive(Copy, Clone)]
enum Instr {
  Lose,
  Draw,
  Win,
}

fn items_to_value(item: Items) -> u32 {
  return match item {
    Items::Rock => 1,
    Items::Paper => 2,
    Items::Scissor => 3,
  };
}

fn items_to_instr(item: Items) -> Instr {
  return match item {
    Items::Rock => Instr::Lose,
    Items::Paper => Instr::Draw,
    Items::Scissor => Instr::Win,
  };
}

fn compare_items(opponent: Items, play: Items) -> u32 {
  let instr = items_to_instr(play);
  return match instr {
    Instr::Draw => items_to_value(opponent) + 3,

    Instr::Lose => match opponent {
      Items::Rock => items_to_value(Items::Scissor),
      Items::Paper => items_to_value(Items::Rock),
      Items::Scissor => items_to_value(Items::Paper),
    },

    Instr::Win => {
      6 + match opponent {
        Items::Scissor => items_to_value(Items::Rock),
        Items::Rock => items_to_value(Items::Paper),
        Items::Paper => items_to_value(Items::Scissor),
      }
    }
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
  let res: u32 = lists.into_iter().map(|x| compare_items(x[0], x[1])).sum();
  println!("{}", res)
}
