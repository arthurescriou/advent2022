use std::env;
use std::fs;

fn main() {
  let file_path = "./input";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let rows: Vec<&str> = contents.trim().split("\n").collect();

  let size = rows.len() / 3 as usize;
  let res: u32 = (0..size)
    .map(|i| {
      [rows[3 * i], rows[3 * i + 1], rows[3 * i + 2]]
        .into_iter()
        .collect()
    })
    .map(|tab| get_val(tab))
    .sum();

  println!("{}", res)
}

fn letter_to_val(letter: char) -> u32 {
  let code = letter as u32;
  let a = 'a' as u32;
  if code < a {
    return code - 'A' as u32 as u32 + 27;
  }
  return code - 'a' as u32 + 1;
}

fn get_val(tab: Vec<&str>) -> u32 {
  let p1 = tab[0];
  let p2 = tab[1];
  let p3 = tab[2];
  let letter = p1
    .chars()
    .filter(|c1| p2.chars().position(|c2| *c1 == c2) != None)
    .fold('a', |acc, val| {
      if p3.chars().position(|c| c == val) == None {
        acc
      } else {
        val
      }
    });
  return letter_to_val(letter);
}
